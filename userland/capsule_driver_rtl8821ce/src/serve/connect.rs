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

//! Joining and leaving a network. A connect finds the target's beacon on the air,
//! runs the whole WPA2 join through the association driver, then installs the
//! negotiated keys into the hardware CAM and records the association so the data
//! path encrypts from them. A disconnect drops the association and clears the keys.

use nonos_libc::crypto_random;
use nonos_wifi_core::dot11::parse::parse_beacon;
use nonos_wifi_core::key::KeyStore;

use crate::assoc::{self, Outcome};
use crate::constants::regs::{NETTYPE_MASK, NETTYPE_SHIFT, NET_TYPE_LINKED, REG_BSSID, REG_CR};
use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::phy::channel::{set_rf, Bw};
use crate::regs::{Mmio, Regs};
use crate::status;

use super::radio::read_mac;
use super::{SCAN_CHANNELS, SCAN_FRAME_MAX};

/// The pairwise and group key slots a connection installs into the CAM.
const PAIRWISE_KEY_ID: u8 = 0;
const GROUP_KEY_ID: u8 = 1;
/// Receive passes to spend joining before giving up. The driver's clock is not
/// reliable in this capsule (all its other timeouts are poll counts), so the
/// join is bounded in passes. Large enough to cover authentication, association
/// and the four-way handshake with the association driver's retransmits, while
/// still returning well within the panel's connect IPC timeout.
const JOIN_BUDGET: u32 = 3_000_000;
/// Receive passes to dwell on each channel while hunting the target's beacon.
/// Long enough that a beacon (sent about every 100 ms) lands while tuned there.
const BEACON_HUNT_DWELL: u32 = 400_000;

/// What the current association installed, so a disconnect clears exactly it.
#[derive(Clone, Copy)]
pub(super) struct Session {
    bssid: [u8; 6],
    key_id: u8,
    gtk_id: u8,
}

/// A connection attempt's result: the status code, and the handshake progress
/// (frames sent and received, and the state reached) for the panel to show.
pub(super) struct ConnectResult {
    pub(super) code: i32,
    pub(super) sent: u32,
    pub(super) recv: u32,
    pub(super) data: u32,
    pub(super) eapol: u32,
    pub(super) probe: u32,
    pub(super) deauth: u32,
    pub(super) to_us: u32,
    pub(super) state: u8,
}

/// Join the network named in the request. The body is the SSID and passphrase,
/// length-prefixed: `[ssid_len][ssid][pass_len][pass]`. Returns 0 on success, or a
/// negative code for a malformed request, no such network, a refused association or
/// a broken handshake, together with how far the handshake got.
pub(super) fn connect(
    body: &[u8],
    link: &mut RtlLink<Regs, Grant, Grant>,
    keys: &mut RtlKeys<Regs>,
    regs: &Regs,
    session: &mut Option<Session>,
) -> ConnectResult {
    let Some((ssid, pass)) = parse_connect(body) else {
        return ConnectResult {
            code: -1,
            sent: 0,
            recv: 0,
            data: 0,
            eapol: 0,
            probe: 0,
            deauth: 0,
            to_us: 0,
            state: 0,
        };
    };
    // Find the network's beacon on the air, which also tells us its channel.
    let mut beacon = [0u8; SCAN_FRAME_MAX];
    let (found, beacons_heard) = find_beacon(link, regs, ssid, &mut beacon);
    let Some((beacon_len, channel)) = found else {
        status::debug(b"[rtl8821ce] connect: network not found\n");
        // recv carries how many beacons of any network were heard during the
        // hunt: 0 means the radio heard nothing (receive/tuning problem), a
        // non-zero count means beacons arrive but the target's did not.
        return ConnectResult {
            code: -2,
            sent: 0,
            recv: beacons_heard,
            data: 0,
            eapol: 0,
            probe: 0,
            deauth: 0,
            to_us: 0,
            state: 0,
        };
    };
    set_rf(regs, channel, Bw::W20);

    // Link the MAC to this BSS before the handshake. The beacon's third address
    // is the BSSID. Until the hardware is told the BSSID and put into managed
    // mode it stays no-link: it hears beacons and the association response
    // (management), but drops the access point's unicast EAPOL data frames, so
    // the four-way handshake never starts. This is why association reached st=3
    // yet no EAPOL was ever counted.
    let mut bssid = [0u8; 6];
    bssid.copy_from_slice(&beacon[16..22]);
    set_media_connected(regs, &bssid);

    let our_mac = read_mac(regs);
    let mut snonce = [0u8; 32];
    crypto_random(snonce.as_mut_ptr(), snonce.len());
    status::debug(b"[rtl8821ce] connect: joining\n");
    let report = assoc::run(link, our_mac, ssid, pass, &beacon[..beacon_len], snonce, JOIN_BUDGET);
    log_progress(&report);
    let (sent, recv, data, eapol, probe, deauth, to_us, state) = (
        report.sent,
        report.recv,
        report.data,
        report.eapol,
        report.probe,
        report.deauth,
        report.to_us,
        report.state,
    );
    let code = match report.outcome {
        Outcome::Joined { bssid, channel, ptk, gtk } => {
            set_rf(regs, channel, Bw::W20);
            if !keys.install_ptk(&ptk, PAIRWISE_KEY_ID, &bssid) {
                -3
            } else if !keys.install_gtk(&gtk, GROUP_KEY_ID) {
                -4
            } else {
                // The keys are in the CAM, so turn the sec engine on for receive
                // decryption (this chip decrypts received frames correctly). The
                // station encrypts transmit frames in software and sends them
                // through the plaintext path the handshake proved works, because
                // the chip's hardware transmit encryption sent frames protected
                // but unencrypted and the access point dropped them.
                crate::sec::enable_sec_engine(regs);
                link.associate(bssid, ptk);
                *session = Some(Session { bssid, key_id: PAIRWISE_KEY_ID, gtk_id: GROUP_KEY_ID });
                status::debug(b"[rtl8821ce] connect: associated\n");
                0
            }
        }
        Outcome::Refused => {
            status::debug(b"[rtl8821ce] connect: refused\n");
            -5
        }
        Outcome::TimedOut => {
            status::debug(b"[rtl8821ce] connect: timed out\n");
            -6
        }
    };
    ConnectResult { code, sent, recv, data, eapol, probe, deauth, to_us, state }
}

/// Drop the association and clear the keys it installed.
pub(super) fn disconnect(
    link: &mut RtlLink<Regs, Grant, Grant>,
    keys: &mut RtlKeys<Regs>,
    session: &mut Option<Session>,
) -> i32 {
    link.deassociate();
    if let Some(s) = session.take() {
        keys.remove_key(s.key_id, Some(&s.bssid));
        keys.remove_key(s.gtk_id, None);
    }
    0
}

// The handshake progress as a debug line: frames sent and received, data and
// EAPOL frames seen, and the state reached, so a failed join names where it
// stopped. Silenced by default; the panel carries the same numbers.
fn log_progress(report: &assoc::Report) {
    status::debug(b"[rtl8821ce] connect: sent=");
    status::debug_number(report.sent);
    status::debug(b" recv=");
    status::debug_number(report.recv);
    status::debug(b" data=");
    status::debug_number(report.data);
    status::debug(b" eapol=");
    status::debug_number(report.eapol);
    status::debug(b" state=");
    status::debug_number(report.state as u32);
    status::debug(b"\n");
}

// Parse the connect request body: [ssid_len][ssid][pass_len][pass].
fn parse_connect(body: &[u8]) -> Option<(&[u8], &[u8])> {
    let ssid_len = *body.first()? as usize;
    let ssid = body.get(1..1 + ssid_len)?;
    let pass_off = 1 + ssid_len;
    let pass_len = *body.get(pass_off)? as usize;
    let pass = body.get(pass_off + 1..pass_off + 1 + pass_len)?;
    Some((ssid, pass))
}

// Hunt the channels for a beacon matching `ssid`, copying the raw frame into `out`
// and returning its length and the channel it was heard on. The join needs the
// real beacon (not the scan summary) because the state machine reads the BSSID,
// channel and RSN element straight out of it.
// Put the MAC into infrastructure "linked" mode for one BSS so the receiver
// accepts the access point's unicast data frames (the EAPOL handshake) addressed
// to us. Writes the BSSID register and sets the network-type field of REG_CR to
// managed/linked, preserving the TX/RX engine enables in that register's low
// bytes. Mirrors rtw88's PORT_SET_BSSID + PORT_SET_NET_TYPE(RTW_NET_MGD_LINKED).
fn set_media_connected(regs: &Regs, bssid: &[u8; 6]) {
    let lo = u32::from_le_bytes([bssid[0], bssid[1], bssid[2], bssid[3]]);
    let hi = u16::from_le_bytes([bssid[4], bssid[5]]);
    regs.write32(REG_BSSID, lo);
    regs.write16(REG_BSSID + 4, hi);

    // Set the network type to managed/linked so the hardware accepts the access
    // point's unicast data frames. This alone gave the most data reception on
    // hardware; enabling the security engine here made the chip try to decrypt
    // every frame and drop what it could not (no key exists yet during the
    // handshake), so it is left off until keys are installed.
    let cr = regs.read32(REG_CR);
    regs.write32(REG_CR, (cr & !NETTYPE_MASK) | (NET_TYPE_LINKED << NETTYPE_SHIFT));
}

// Returns the matched beacon (length and channel) if found, together with the
// number of beacons of ANY network heard during the hunt. That count turns a
// bare "not found" into a diagnosis: zero means the radio heard nothing at all
// on any channel (a receive-path or tuning problem), non-zero means beacons are
// arriving but the target's was not among them (wrong SSID or it was off-air).
fn find_beacon(
    link: &mut RtlLink<Regs, Grant, Grant>,
    regs: &Regs,
    ssid: &[u8],
    out: &mut [u8; SCAN_FRAME_MAX],
) -> (Option<(usize, u8)>, u32) {
    let mut frame = [0u8; SCAN_FRAME_MAX];
    let mut beacons = 0u32;
    for &ch in &SCAN_CHANNELS {
        set_rf(regs, ch, Bw::W20);
        for _ in 0..BEACON_HUNT_DWELL {
            let Some(n) = link.poll_raw(&mut frame) else {
                continue;
            };
            if let Some(info) = parse_beacon(&frame[..n]) {
                beacons += 1;
                if info.ssid == ssid {
                    let channel =
                        if info.channel >= 1 && info.channel <= 14 { info.channel } else { ch };
                    out[..n].copy_from_slice(&frame[..n]);
                    return (Some((n, channel)), beacons);
                }
            }
        }
    }
    (None, beacons)
}
