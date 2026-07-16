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

//! The WPA2 data plane: convert an ethernet frame to an 802.11 data frame and
//! back, so once the link is up the IP stack rides the radio unchanged.
//!
//! Framing and encryption are split so a chip that does CCMP in hardware and a
//! chip that does it in software share the same code without one double
//! encrypting:
//!
//!   * `build_data` produces the plaintext 802.11 MPDU (MAC header, LLC/SNAP,
//!     ethertype, payload). A hardware-CCMP chip hands exactly this to the
//!     radio, which inserts the CCMP header and encrypts from its key cache.
//!   * `protect` is software CCMP: it takes that plaintext MPDU and returns the
//!     encrypted frame. A chip without crypto uses `build_data` then `protect`.
//!   * `unprotect` is the software receive half: decrypt and strip CCMP,
//!     yielding a plaintext MPDU; a hardware-CCMP chip skips it.
//!   * `parse_data` turns a plaintext MPDU back into an ethernet frame.
//!
//! `encap`/`decap` remain as the software round trip (`build_data` + `protect`,
//! `unprotect` + `parse_data`) so their byte output is unchanged. Encryption is
//! CCMP (AES-CCM with an 802.11-specific nonce and AAD built from the MAC
//! header) keyed by the pairwise temporal key. The AES-CCM core is proven
//! against RFC 3610; the nonce/AAD construction is asserted byte for byte in
//! `iwlwifi_proofs`.

use alloc::vec::Vec;

use super::header::{
    frame_control, header_len, seq_control, MacAddr, FC_FROM_DS, FC_PROTECTED, FC_TO_DS,
    MAC_HEADER_LEN, TYPE_DATA,
};
use crate::ccmp::ccm::{ccm_decrypt, ccm_encrypt};

// LLC/SNAP header that precedes the ethertype in an 802.11 data payload.
const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];
const CCMP_HDR_LEN: usize = 8;
const MIC_LEN: usize = 8;
const ETH_HEADER_LEN: usize = 14;

/// Build the plaintext 802.11 data MPDU for an ethernet frame: the ToDS MAC
/// header with the Protected bit clear, the LLC/SNAP shim, the ethertype and
/// the payload. `our_mac` is the station, `bssid` the AP. This is the frame a
/// hardware-CCMP chip is handed directly (it inserts the CCMP header and
/// encrypts from its key cache); a software-CCMP chip passes it to `protect`.
/// Returns `None` if the ethernet frame is malformed.
pub fn build_data(eth: &[u8], our_mac: MacAddr, bssid: MacAddr, seq: u16) -> Option<Vec<u8>> {
    if eth.len() < ETH_HEADER_LEN {
        return None;
    }
    let mut dst = [0u8; 6];
    dst.copy_from_slice(&eth[0..6]);
    let fc = frame_control(TYPE_DATA, 0) | FC_TO_DS;
    let sc = seq_control(seq);
    // ToDS addressing: addr1 = BSSID, addr2 = source (us), addr3 = destination.
    let mut mpdu = Vec::with_capacity(MAC_HEADER_LEN + 8 + eth.len() - ETH_HEADER_LEN);
    mpdu.extend_from_slice(&fc.to_le_bytes());
    mpdu.extend_from_slice(&[0, 0]); // duration
    mpdu.extend_from_slice(&bssid);
    mpdu.extend_from_slice(&our_mac);
    mpdu.extend_from_slice(&dst);
    mpdu.extend_from_slice(&sc.to_le_bytes());
    // Plaintext body: LLC/SNAP, the ethertype, then the ethernet payload.
    mpdu.extend_from_slice(&LLC_SNAP);
    mpdu.extend_from_slice(&eth[12..14]);
    mpdu.extend_from_slice(&eth[ETH_HEADER_LEN..]);
    Some(mpdu)
}

/// Apply software CCMP to a plaintext MPDU from `build_data`: set the Protected
/// bit, insert the CCMP header for packet number `pn`, and replace the body with
/// its AES-CCM ciphertext and MIC keyed by `tk`. This is the path for a chip
/// without hardware crypto; a hardware-CCMP chip never calls it. Returns `None`
/// if the MPDU is shorter than a MAC header.
pub fn protect(mpdu: &[u8], pn: u64, tk: &[u8; 16]) -> Option<Vec<u8>> {
    if mpdu.len() < MAC_HEADER_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]) | FC_PROTECTED;
    let mut a1 = [0u8; 6];
    let mut a2 = [0u8; 6];
    let mut a3 = [0u8; 6];
    a1.copy_from_slice(&mpdu[4..10]);
    a2.copy_from_slice(&mpdu[10..16]);
    a3.copy_from_slice(&mpdu[16..22]);
    let sc = u16::from_le_bytes([mpdu[22], mpdu[23]]);
    let plain = &mpdu[MAC_HEADER_LEN..];

    let mut frame = Vec::with_capacity(MAC_HEADER_LEN + CCMP_HDR_LEN + plain.len() + MIC_LEN);
    frame.extend_from_slice(&fc.to_le_bytes());
    frame.extend_from_slice(&mpdu[2..4]); // duration, preserved
    frame.extend_from_slice(&a1);
    frame.extend_from_slice(&a2);
    frame.extend_from_slice(&a3);
    frame.extend_from_slice(&mpdu[22..24]); // sequence control, preserved
    frame.extend_from_slice(&ccmp_header(pn));

    let aad = build_aad(fc, &a1, &a2, &a3, sc);
    let nonce = build_nonce(&a2, pn);
    let mut ct = alloc::vec![0u8; plain.len() + MIC_LEN];
    let n = ccm_encrypt(tk, &nonce, &aad, plain, &mut ct)?;
    frame.extend_from_slice(&ct[..n]);
    Some(frame)
}

/// Encrypt an ethernet frame into an 802.11 data frame addressed to the AP
/// (ToDS) at packet number `pn`, the software CCMP round trip. Equivalent to
/// `build_data` followed by `protect`; a hardware-CCMP chip uses `build_data`
/// alone. Returns `None` if the ethernet frame is malformed.
pub fn encap(
    eth: &[u8],
    our_mac: MacAddr,
    bssid: MacAddr,
    pn: u64,
    tk: &[u8; 16],
    seq: u16,
) -> Option<Vec<u8>> {
    protect(&build_data(eth, our_mac, bssid, seq)?, pn, tk)
}

/// Remove software CCMP from an 802.11 data frame: verify and strip the CCMP
/// header and MIC, returning the plaintext MPDU with the Protected bit cleared.
/// A hardware-CCMP chip delivers an already-decrypted MPDU and skips this,
/// handing the frame straight to `parse_data`. Returns `None` if the frame is
/// too short or the MIC fails.
pub fn unprotect(frame: &[u8], tk: &[u8; 16]) -> Option<Vec<u8>> {
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

    let mut mpdu = Vec::with_capacity(MAC_HEADER_LEN + n);
    mpdu.extend_from_slice(&(fc & !FC_PROTECTED).to_le_bytes());
    mpdu.extend_from_slice(&frame[2..4]); // duration
    mpdu.extend_from_slice(&a1);
    mpdu.extend_from_slice(&a2);
    mpdu.extend_from_slice(&a3);
    mpdu.extend_from_slice(&frame[22..24]); // sequence control
    mpdu.extend_from_slice(&plain[..n]);
    Some(mpdu)
}

/// Convert a plaintext 802.11 data MPDU (MAC header, LLC/SNAP, ethertype,
/// payload) into an ethernet frame, handling both the AP-to-station (FromDS)
/// and station-to-AP (ToDS) address layouts. This is the receive path after
/// decryption, whether by software `unprotect` or a hardware-CCMP chip that
/// delivered the frame plaintext. Returns `None` if the MPDU is too short or
/// its body is not LLC/SNAP encapsulated.
pub fn parse_data(mpdu: &[u8]) -> Option<Vec<u8>> {
    if mpdu.len() < MAC_HEADER_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
    // A QoS data frame carries a longer header; size the body from the frame so
    // the LLC/SNAP shim is found whether or not the AP used QoS.
    let hlen = header_len(fc);
    // A hardware-CCMP chip delivers a decrypted frame with the 802.11 header and
    // the 8-byte CCMP header (IV and packet number) still in place, and leaves the
    // Protected bit set in the frame control. The plaintext body begins after both
    // headers. Skipping the CCMP header when protected is what puts the LLC/SNAP
    // shim at the expected offset; without it every hardware-decrypted frame, the
    // DHCP reply included, failed the LLC/SNAP check and was silently dropped.
    let ccmp = if fc & FC_PROTECTED != 0 { CCMP_HDR_LEN } else { 0 };
    if mpdu.len() < hlen + ccmp + 8 {
        return None;
    }
    let mut a1 = [0u8; 6];
    let mut a2 = [0u8; 6];
    let mut a3 = [0u8; 6];
    a1.copy_from_slice(&mpdu[4..10]);
    a2.copy_from_slice(&mpdu[10..16]);
    a3.copy_from_slice(&mpdu[16..22]);
    let body = &mpdu[hlen + ccmp..];
    if body.len() < 8 || body[0..6] != LLC_SNAP {
        return None;
    }
    // Destination and source depend on the DS bits: FromDS addresses the
    // station as addr1 with the source in addr3; ToDS carries the destination
    // in addr3 and the source in addr2.
    let (da, sa) = if fc & FC_FROM_DS != 0 { (a1, a3) } else { (a3, a2) };
    let mut eth = Vec::with_capacity(ETH_HEADER_LEN + body.len() - 8);
    eth.extend_from_slice(&da);
    eth.extend_from_slice(&sa);
    eth.extend_from_slice(&body[6..8]); // ethertype
    eth.extend_from_slice(&body[8..]); // payload
    Some(eth)
}

/// Decrypt an 802.11 data frame back into an ethernet frame, the software CCMP
/// round trip. Equivalent to `unprotect` followed by `parse_data`; a
/// hardware-CCMP chip uses `parse_data` alone. Returns `None` if the frame is
/// too short, the MIC fails, or the body is not LLC/SNAP encapsulated.
pub fn decap(frame: &[u8], tk: &[u8; 16]) -> Option<Vec<u8>> {
    parse_data(&unprotect(frame, tk)?)
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
