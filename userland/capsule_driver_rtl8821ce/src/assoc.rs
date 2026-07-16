// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Join a WPA2 network. The shared `Mlme` state machine owns all the protocol and
//! crypto: it takes the SSID and passphrase, and given the frames received while
//! joining it produces the frames to send and finally the pairwise and group
//! keys. This module is the radio half: it feeds received frames to the machine
//! (management frames drive scan/auth/assoc, EAPOL data frames drive the four-way
//! handshake) and transmits what the machine returns (management frames go out
//! raw, EAPOL replies are wrapped into an unencrypted 802.11 data frame, since the
//! keys do not exist yet). The frame classification and the EAPOL wrapping are
//! pure and checked on the host; only the transmit and receive calls touch a
//! radio.

use alloc::vec::Vec;

use nonos_wifi_core::dot11::data::build_data;
use nonos_wifi_core::dot11::header::{MAC_HEADER_LEN, TYPE_DATA, TYPE_MGMT};
use nonos_wifi_core::frame::LLC_SNAP;
use nonos_wifi_core::mlme::{Mlme, MlmeState};

/// The EtherType that marks an 802.1X (EAPOL) payload.
const ETHERTYPE_EAPOL: u16 = 0x888E;
/// An ethernet header is two MACs and the ethertype.
const ETH_HEADER_LEN: usize = 14;
/// Idle receive passes before the last transmitted frame is resent. A Wi-Fi link
/// loses frames and the state machine sends each once, so a lost authentication
/// or association frame is retransmitted this often until the peer replies. The
/// driver's clock is not reliable in this capsule (it uses poll counts for all
/// its other timeouts), so the join is bounded in passes, not wall time.
const RETX_AFTER: u32 = 200_000;

/// The radio the association driver transmits on and receives from. The driver
/// implements this over its transmit and receive rings; the proofs implement it
/// over a scripted exchange.
pub trait Radio {
    /// Transmit one complete 802.11 frame (management or data). Returns false if
    /// it could not be queued.
    fn send(&mut self, mpdu: &[u8]) -> bool;
    /// Copy the next received 802.11 frame into `out`, returning its length, or
    /// `None` if nothing is waiting right now.
    fn recv(&mut self, out: &mut [u8]) -> Option<usize>;
}

/// How a join attempt ended.
pub enum Outcome {
    /// Associated: the pairwise and group keys and the AP to install them for.
    Joined { bssid: [u8; 6], channel: u8, ptk: [u8; 16], gtk: [u8; 16] },
    /// The association was refused or the handshake broke.
    Refused,
    /// The AP stopped responding before the join completed.
    TimedOut,
}

/// The join outcome together with how far it got: frames transmitted, frames
/// received, and the state the machine reached. The counts turn a bare timeout
/// into a diagnosis, one frame sent and none received points at transmit, several
/// exchanged points past it.
pub struct Report {
    pub outcome: Outcome,
    pub sent: u32,
    pub recv: u32,
    /// Data frames delivered off the ring. Distinguishes "the card never gave us a
    /// data frame" (a receive-filter or link-state problem) from "data frames
    /// arrive but none parse as EAPOL" (a framing problem).
    pub data: u32,
    /// EAPOL key frames recognized off the air. Zero while `state` is FourWay with
    /// data frames arriving means the handshake frame is not being parsed as EAPOL;
    /// zero with no data frames means it never reached us.
    pub eapol: u32,
    /// The first non-EAPOL data frame's frame-control (low 16) and length (high 16).
    pub probe: u32,
    /// Deauthenticate/disassociate frames received. Non-zero means the access
    /// point started the connection then tore it down, which it does after it
    /// sends the first EAPOL frame and gets no reply: proof the AP is trying to
    /// key us and the handshake frame is being lost on our side.
    pub deauth: u32,
    /// Data frames whose receiver address is our own MAC (unicast to us) rather
    /// than a group address. The EAPOL handshake arrives this way, so zero here
    /// while broadcast data arrives means unicast data is being dropped, not that
    /// the AP is silent.
    pub to_us: u32,
    pub state: u8,
}

/// The state machine's progress as a single byte, for reporting.
fn state_code(s: MlmeState) -> u8 {
    match s {
        MlmeState::Scanning => 0,
        MlmeState::Authenticating => 1,
        MlmeState::Associating => 2,
        MlmeState::FourWay => 3,
        MlmeState::Connected => 4,
        MlmeState::Failed => 5,
    }
}

/// What a received frame is, to the join state machine.
#[derive(Debug, PartialEq, Eq)]
pub enum RxKind {
    /// A management frame, fed to the machine whole.
    Mgmt,
    /// A data frame carrying an EAPOL payload at this byte range.
    Eapol(usize, usize),
    /// Anything else: ignored while joining.
    Other,
}

/// The frame control field's type bits.
fn fc_type(frame: &[u8]) -> Option<u8> {
    if frame.len() < 2 {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    Some(((fc >> 2) & 0x3) as u8)
}

/// Classify a received 802.11 frame for the join. A management frame is handed to
/// the machine whole; a data frame is unwrapped and, if it carries EAPOL, its
/// payload range within `eth` is returned so the caller can feed it.
pub fn classify(frame: &[u8], eth: &mut Vec<u8>) -> RxKind {
    match fc_type(frame) {
        Some(TYPE_MGMT) => RxKind::Mgmt,
        Some(TYPE_DATA) => match eapol_payload_start(frame) {
            Some(pos) => {
                eth.clear();
                eth.extend_from_slice(&frame[pos..]);
                RxKind::Eapol(0, eth.len())
            }
            None => RxKind::Other,
        },
        _ => RxKind::Other,
    }
}

/// Where the EAPOL payload begins in a data frame, or `None` if it carries none.
/// Rather than compute the header length (which varies with QoS, HT-control and
/// four-address frames, and which an access point can combine in ways a fixed
/// offset gets wrong), this searches for the LLC/SNAP shim followed by the EAPOL
/// ethertype anywhere past the MAC header. The handshake frames are unencrypted,
/// so the signature appears in the clear.
fn eapol_payload_start(frame: &[u8]) -> Option<usize> {
    if frame.len() < MAC_HEADER_LEN + 8 {
        return None;
    }
    for pos in MAC_HEADER_LEN..=frame.len() - 8 {
        if frame[pos..pos + 6] == LLC_SNAP
            && u16::from_be_bytes([frame[pos + 6], frame[pos + 7]]) == ETHERTYPE_EAPOL
        {
            return Some(pos + 8);
        }
    }
    None
}

/// Wrap an EAPOL payload into an unencrypted 802.11 data frame to the AP. The
/// keys are not installed yet, so the handshake frames go out in the clear, which
/// is exactly what the standard expects. Builds the ethernet frame the shared
/// encapsulator wants, then hands it to `build_data`.
pub fn wrap_eapol(eapol: &[u8], our_mac: [u8; 6], bssid: [u8; 6], seq: u16) -> Option<Vec<u8>> {
    let mut eth = Vec::with_capacity(ETH_HEADER_LEN + eapol.len());
    eth.extend_from_slice(&bssid); // destination: the AP
    eth.extend_from_slice(&our_mac); // source: the station
    eth.extend_from_slice(&ETHERTYPE_EAPOL.to_be_bytes());
    eth.extend_from_slice(eapol);
    build_data(&eth, our_mac, bssid, seq)
}

/// Run the join to completion, or until `budget` receive passes finish without
/// completing. `beacon` is the cached beacon for the target network, which starts
/// the machine; `snonce` is a fresh random nonce for the handshake.
pub fn run<R: Radio>(
    radio: &mut R,
    our_mac: [u8; 6],
    ssid: &[u8],
    passphrase: &[u8],
    beacon: &[u8],
    snonce: [u8; 32],
    budget: u32,
) -> Report {
    let mut mlme = Mlme::new(our_mac, ssid, passphrase, snonce);
    let mut c = Counters::default();
    // The last frame transmitted, and how long it has gone unanswered. A Wi-Fi link
    // loses frames, and the state machine sends each frame once, so without this a
    // single lost authentication, association or handshake frame hangs the whole
    // join. The pending frame is resent after `RETX_AFTER` idle passes. It is
    // cleared whenever the state advances without producing a reply, because that
    // means we are now waiting for the peer to send next (the access point starts
    // the four-way handshake), and resending our last frame would only churn.
    let mut pending: Vec<u8> = Vec::new();
    let mut idle: u32 = 0;

    // The beacon selects the BSS and produces the authentication request.
    let out = mlme.on_mgmt(beacon);
    if let Some(tx) = out.tx {
        if radio.send(&tx) {
            c.sent += 1;
        }
        pending = tx;
    }

    let mut seq: u16 = 0;
    let mut frame = [0u8; 2048];
    let mut eth = Vec::new();
    for _ in 0..budget {
        match mlme.state() {
            MlmeState::Connected => return report(&mlme, finish(&mlme), c),
            MlmeState::Failed => return report(&mlme, Outcome::Refused, c),
            _ => {}
        }
        idle += 1;
        if idle >= RETX_AFTER && !pending.is_empty() {
            if radio.send(&pending) {
                c.sent += 1;
            }
            idle = 0;
        }
        let Some(n) = radio.recv(&mut frame) else {
            continue;
        };
        c.recv += 1;
        let rx = &frame[..n];
        let before = mlme.state();
        let tx = handle_frame(&mut mlme, rx, &mut eth, &mut c, &mut seq);
        match tx {
            // Produced a reply: transmit it and make it the frame to retransmit.
            Some(mpdu) => {
                if radio.send(&mpdu) {
                    c.sent += 1;
                }
                pending = mpdu;
                idle = 0;
            }
            // No reply, but the state advanced: we now wait for the peer, so stop
            // resending the previous frame.
            None if mlme.state() != before => {
                pending.clear();
                idle = 0;
            }
            None => {}
        }
    }
    if mlme.state() == MlmeState::Connected {
        return report(&mlme, finish(&mlme), c);
    }
    report(&mlme, Outcome::TimedOut, c)
}

/// The running progress counters for one join.
#[derive(Default)]
struct Counters {
    sent: u32,
    recv: u32,
    data: u32,
    eapol: u32,
    /// The first non-EAPOL data frame's frame-control in the low 16 bits and its
    /// length in the high 16, for diagnosis: the protected bit and subtype in the
    /// frame-control, and whether the length is handshake-sized, say what the frame
    /// the card delivered actually is.
    probe: u32,
    /// Deauthenticate/disassociate management frames received.
    deauth: u32,
    /// Data frames addressed (receiver address) to our own MAC.
    to_us: u32,
}

// Feed one received frame to the machine and return the frame to transmit next,
// if any (an EAPOL reply is wrapped into an 802.11 data frame). Also counts the
// data and EAPOL frames seen, for diagnosis.
fn handle_frame(
    mlme: &mut Mlme,
    rx: &[u8],
    eth: &mut Vec<u8>,
    c: &mut Counters,
    seq: &mut u16,
) -> Option<Vec<u8>> {
    match classify(rx, eth) {
        RxKind::Mgmt => {
            // A deauthenticate (subtype 0xC) or disassociate (0xA) means the AP
            // set the connection up then dropped it, which is what it does after
            // sending an EAPOL frame it never gets a reply to.
            if !rx.is_empty() {
                let subtype = rx[0] & 0xF0;
                if subtype == 0xC0 || subtype == 0xA0 {
                    c.deauth += 1;
                }
            }
            mlme.on_mgmt(rx).tx
        }
        RxKind::Eapol(start, end) => {
            c.eapol += 1;
            let reply = mlme.on_eapol(&eth[start..end]).tx?;
            let mpdu = wrap_eapol(&reply, mlme.our_mac(), mlme.bssid(), *seq)?;
            *seq = seq.wrapping_add(1);
            Some(mpdu)
        }
        RxKind::Other => {
            // Count any data frame the card delivered, EAPOL or not, so an empty
            // EAPOL count can be told apart from no data reaching us at all. For
            // the probe, prefer the first UNPROTECTED data frame (the shape an
            // EAPOL msg1 arrives in), upgrading from an earlier protected capture:
            // the access point's encrypted group traffic is protected, so if the
            // probe stays protected no plaintext EAPOL ever reached us, whereas an
            // unprotected fc points the finger at the parser instead.
            if fc_type(rx) == Some(TYPE_DATA) && rx.len() >= 2 {
                c.data += 1;
                // Receiver address (addr1) is bytes 4..10. Count frames unicast to
                // our own MAC: the EAPOL handshake arrives that way.
                if rx.len() >= 10 && rx[4..10] == mlme.our_mac() {
                    c.to_us += 1;
                }
                let fc = u16::from_le_bytes([rx[0], rx[1]]);
                let unprotected = fc & 0x4000 == 0;
                let have_unprotected = c.probe != 0 && (c.probe & 0x4000) == 0;
                if c.probe == 0 || (unprotected && !have_unprotected) {
                    c.probe = fc as u32 | ((rx.len() as u32 & 0xFFFF) << 16);
                }
            }
            None
        }
    }
}

// Wrap an outcome with the progress counters and the final state.
fn report(mlme: &Mlme, outcome: Outcome, c: Counters) -> Report {
    Report {
        outcome,
        sent: c.sent,
        recv: c.recv,
        data: c.data,
        eapol: c.eapol,
        probe: c.probe,
        deauth: c.deauth,
        to_us: c.to_us,
        state: state_code(mlme.state()),
    }
}

// Pull the negotiated keys out of a connected machine.
fn finish(mlme: &Mlme) -> Outcome {
    let (Some(tk), Some(gtk)) = (mlme.tk(), mlme.gtk()) else {
        return Outcome::Refused;
    };
    let mut ptk = [0u8; 16];
    let mut group = [0u8; 16];
    if tk.len() < 16 || gtk.len() < 16 {
        return Outcome::Refused;
    }
    ptk.copy_from_slice(&tk[..16]);
    group.copy_from_slice(&gtk[..16]);
    Outcome::Joined { bssid: mlme.bssid(), channel: mlme.channel(), ptk, gtk: group }
}
