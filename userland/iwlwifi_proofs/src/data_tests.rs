// NONOS Operating System (AGPL-3.0-or-later)
//! Proof for the WPA2 CCMP data plane. The AES-CCM core is already checked
//! against RFC 3610; this proof checks the 802.11-specific framing around it:
//! the nonce and additional-authenticated-data are asserted byte for byte
//! against the IEEE layout (so a construction error cannot hide behind a
//! self-consistent round trip), an ethernet frame survives an
//! encrypt-then-decrypt round trip unchanged, and a single flipped ciphertext
//! byte is rejected by the MIC. No hardware.

use crate::dot11::data::{build_aad, build_nonce, decap, encap};
use crate::dot11::header::{frame_control, seq_control, TYPE_DATA};

const OUR: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
const BSSID: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
const DST: [u8; 6] = [0x33, 0x33, 0x00, 0x00, 0x00, 0x16];
const TK: [u8; 16] = [
    0xc9, 0x7c, 0x1f, 0x67, 0xce, 0x37, 0x11, 0x85, 0x51, 0x4a, 0x8a, 0x19, 0xf2, 0xbd, 0xd5, 0x2f,
];

const FC_TODS: u16 = 0x0100;
const FC_PROTECTED: u16 = 0x4000;

#[test]
fn aad_matches_the_ieee_layout() {
    let fc = frame_control(TYPE_DATA, 0) | FC_TODS | FC_PROTECTED; // 0x4108
    let sc = seq_control(5); // sequence 5 -> masked to fragment 0 in the AAD
    let aad = build_aad(fc, &BSSID, &OUR, &DST, sc);
    let mut want = [0u8; 22];
    want[0..2].copy_from_slice(&fc.to_le_bytes()); // no masked bits set here
    want[2..8].copy_from_slice(&BSSID);
    want[8..14].copy_from_slice(&OUR);
    want[14..20].copy_from_slice(&DST);
    // sequence number masked out, fragment number zero
    assert_eq!(aad, want, "AAD is frame control, three addresses, masked sequence");
}

#[test]
fn nonce_matches_the_ieee_layout() {
    let pn: u64 = 0x00_00_00_01_02_03;
    let nonce = build_nonce(&OUR, pn);
    let mut want = [0u8; 13];
    want[0] = 0; // priority octet, non-QoS data
    want[1..7].copy_from_slice(&OUR); // transmitter address
    // packet number, most significant octet first
    want[7..13].copy_from_slice(&[0x00, 0x00, 0x00, 0x01, 0x02, 0x03]);
    assert_eq!(nonce, want, "nonce is priority, A2, then PN big-endian");
}

fn ethernet_frame() -> Vec<u8> {
    // dst, src (= our MAC, as the stack sends), ethertype (IPv4), payload.
    let mut f = Vec::new();
    f.extend_from_slice(&DST);
    f.extend_from_slice(&OUR);
    f.extend_from_slice(&[0x08, 0x00]);
    f.extend_from_slice(b"hello over the air, encrypted end to end");
    f
}

#[test]
fn ethernet_survives_an_encrypt_decrypt_round_trip() {
    let eth = ethernet_frame();
    let frame = encap(&eth, OUR, BSSID, 1, &TK, 1).expect("encap");
    assert_ne!(&frame[32..], &eth[14..], "the payload is actually encrypted");
    let back = decap(&frame, &TK).expect("decap");
    assert_eq!(back, eth, "the ethernet frame is recovered exactly");
}

#[test]
fn a_flipped_ciphertext_byte_is_rejected() {
    let eth = ethernet_frame();
    let mut frame = encap(&eth, OUR, BSSID, 7, &TK, 7).expect("encap");
    let last = frame.len() - 1;
    frame[last] ^= 0x80; // corrupt the MIC
    assert!(decap(&frame, &TK).is_none(), "a bad MIC is rejected");
    let mid = 40;
    frame[last] ^= 0x80; // restore
    frame[mid] ^= 0x01; // corrupt ciphertext
    assert!(decap(&frame, &TK).is_none(), "tampered ciphertext is rejected");
}
