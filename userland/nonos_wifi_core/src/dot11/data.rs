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

//! The WPA2 data plane: convert an ethernet frame to an encrypted 802.11 data
//! frame and back, so once the link is up the IP stack rides the radio
//! unchanged. Encryption is CCMP (AES-CCM with an 802.11-specific nonce and
//! additional authenticated data built from the MAC header) keyed by the
//! pairwise temporal key from the handshake. The AES-CCM core is proven against
//! RFC 3610; this module is the 802.11 framing around it, and the nonce/AAD
//! construction is asserted byte for byte in `iwlwifi_proofs`.

use alloc::vec::Vec;

use super::header::{frame_control, seq_control, MacAddr, MAC_HEADER_LEN, TYPE_DATA};
use crate::ccmp::ccm::{ccm_decrypt, ccm_encrypt};

// LLC/SNAP header that precedes the ethertype in an 802.11 data payload.
const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];
const CCMP_HDR_LEN: usize = 8;
const MIC_LEN: usize = 8;
const ETH_HEADER_LEN: usize = 14;

// Frame-control flag bits.
const FC_TODS: u16 = 0x0100;
const FC_FROMDS: u16 = 0x0200;
const FC_PROTECTED: u16 = 0x4000;

/// Encrypt an ethernet frame into an 802.11 data frame addressed to the AP
/// (ToDS), advancing and using packet number `pn`. `our_mac` is the station,
/// `bssid` the AP, `tk` the pairwise key. Returns the frame, or `None` if the
/// ethernet frame is malformed.
pub fn encap(
    eth: &[u8],
    our_mac: MacAddr,
    bssid: MacAddr,
    pn: u64,
    tk: &[u8; 16],
    seq: u16,
) -> Option<Vec<u8>> {
    if eth.len() < ETH_HEADER_LEN {
        return None;
    }
    let mut dst = [0u8; 6];
    dst.copy_from_slice(&eth[0..6]);
    // Plaintext: LLC/SNAP, the ethertype, then the ethernet payload.
    let mut plain = Vec::with_capacity(8 + eth.len() - ETH_HEADER_LEN);
    plain.extend_from_slice(&LLC_SNAP);
    plain.extend_from_slice(&eth[12..14]);
    plain.extend_from_slice(&eth[ETH_HEADER_LEN..]);

    let fc = frame_control(TYPE_DATA, 0) | FC_TODS | FC_PROTECTED;
    let sc = seq_control(seq);
    // ToDS addressing: addr1 = BSSID, addr2 = source (us), addr3 = destination.
    let mut frame = Vec::with_capacity(MAC_HEADER_LEN + CCMP_HDR_LEN + plain.len() + MIC_LEN);
    frame.extend_from_slice(&fc.to_le_bytes());
    frame.extend_from_slice(&[0, 0]); // duration
    frame.extend_from_slice(&bssid);
    frame.extend_from_slice(&our_mac);
    frame.extend_from_slice(&dst);
    frame.extend_from_slice(&sc.to_le_bytes());
    frame.extend_from_slice(&ccmp_header(pn));

    let aad = build_aad(fc, &bssid, &our_mac, &dst, sc);
    let nonce = build_nonce(&our_mac, pn);
    let mut ct = alloc::vec![0u8; plain.len() + MIC_LEN];
    let n = ccm_encrypt(tk, &nonce, &aad, &plain, &mut ct)?;
    frame.extend_from_slice(&ct[..n]);
    Some(frame)
}

/// Decrypt an 802.11 data frame back into an ethernet frame. Handles both the
/// AP-to-station (FromDS) and station-to-AP (ToDS) address layouts. Returns
/// `None` if the frame is too short, the MIC fails, or the payload is not
/// LLC/SNAP encapsulated.
pub fn decap(frame: &[u8], tk: &[u8; 16]) -> Option<Vec<u8>> {
    if frame.len() < MAC_HEADER_LEN + CCMP_HDR_LEN + MIC_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    let mut a1 = [0u8; 6];
    let mut a2 = [0u8; 6];
    let mut a3 = [0u8; 6];
    a1.copy_from_slice(&frame[4..10]);
    a2.copy_from_slice(&frame[10..16]);
    a3.copy_from_slice(&frame[16..22]);
    let sc = u16::from_le_bytes([frame[22], frame[23]]);
    let pn = pn_from_header(&frame[MAC_HEADER_LEN..MAC_HEADER_LEN + CCMP_HDR_LEN]);

    let aad = build_aad(fc, &a1, &a2, &a3, sc);
    let nonce = build_nonce(&a2, pn);
    let ct = &frame[MAC_HEADER_LEN + CCMP_HDR_LEN..];
    let mut plain = alloc::vec![0u8; ct.len()];
    let n = ccm_decrypt(tk, &nonce, &aad, ct, &mut plain)?;
    if n < 8 || plain[0..6] != LLC_SNAP {
        return None;
    }
    // Destination and source depend on the DS bits: FromDS addresses the
    // station as addr1 with the source in addr3; ToDS carries the destination
    // in addr3 and the source in addr2.
    let (da, sa) = if fc & FC_FROMDS != 0 { (a1, a3) } else { (a3, a2) };
    let mut eth = Vec::with_capacity(ETH_HEADER_LEN + n - 8);
    eth.extend_from_slice(&da);
    eth.extend_from_slice(&sa);
    eth.extend_from_slice(&plain[6..8]); // ethertype
    eth.extend_from_slice(&plain[8..n]); // payload
    Some(eth)
}

// The 8-octet CCMP header: PN0, PN1, reserved, key-id byte (ExtIV set), then
// PN2..PN5. PN0 is the least significant octet.
pub fn ccmp_header(pn: u64) -> [u8; 8] {
    [
        pn as u8,
        (pn >> 8) as u8,
        0,
        0x20, // ExtIV, key id 0
        (pn >> 16) as u8,
        (pn >> 24) as u8,
        (pn >> 32) as u8,
        (pn >> 40) as u8,
    ]
}

fn pn_from_header(h: &[u8]) -> u64 {
    (h[0] as u64)
        | ((h[1] as u64) << 8)
        | ((h[4] as u64) << 16)
        | ((h[5] as u64) << 24)
        | ((h[6] as u64) << 32)
        | ((h[7] as u64) << 40)
}

/// The 13-octet CCM nonce: a priority octet (zero for non-QoS data), the
/// transmitter address, then the 48-bit packet number most significant octet
/// first.
pub fn build_nonce(a2: &MacAddr, pn: u64) -> [u8; 13] {
    let mut n = [0u8; 13];
    n[1..7].copy_from_slice(a2);
    n[7] = (pn >> 40) as u8;
    n[8] = (pn >> 32) as u8;
    n[9] = (pn >> 24) as u8;
    n[10] = (pn >> 16) as u8;
    n[11] = (pn >> 8) as u8;
    n[12] = pn as u8;
    n
}

/// The 22-octet AAD for a non-QoS data frame: the frame-control field with the
/// retry, power-management, more-data and order subfields masked to zero, the
/// three addresses, and the sequence-control field with the sequence number
/// masked out (only the fragment number retained).
pub fn build_aad(fc: u16, a1: &MacAddr, a2: &MacAddr, a3: &MacAddr, sc: u16) -> [u8; 22] {
    let fc_masked = fc & !(0x0800 | 0x1000 | 0x2000 | 0x8000);
    let sc_masked = sc & 0x000F;
    let mut aad = [0u8; 22];
    aad[0..2].copy_from_slice(&fc_masked.to_le_bytes());
    aad[2..8].copy_from_slice(a1);
    aad[8..14].copy_from_slice(a2);
    aad[14..20].copy_from_slice(a3);
    aad[20..22].copy_from_slice(&sc_masked.to_le_bytes());
    aad
}
