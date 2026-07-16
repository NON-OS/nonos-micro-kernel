// NONOS Operating System (AGPL-3.0-or-later)
//! Proof for the WPA2 CCMP data plane. The AES-CCM core is already checked
//! against RFC 3610; this proof checks the 802.11-specific framing around it:
//! the nonce and additional-authenticated-data are asserted byte for byte
//! against the IEEE layout (so a construction error cannot hide behind a
//! self-consistent round trip), an ethernet frame survives an
//! encrypt-then-decrypt round trip unchanged, and a single flipped ciphertext
//! byte is rejected by the MIC. No hardware.

use crate::dot11::data::{
    build_aad, build_data, build_nonce, decap, encap, parse_data, protect, unprotect,
};
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
fn software_split_composes_to_encap_and_decap() {
    // `encap` must be exactly `build_data` then `protect`, and `decap` exactly
    // `unprotect` then `parse_data`, so splitting the crypto out for
    // hardware-CCMP chips does not change the software round trip by a byte.
    let eth = ethernet_frame();
    let whole = encap(&eth, OUR, BSSID, 3, &TK, 3).expect("encap");
    let split = protect(&build_data(&eth, OUR, BSSID, 3).expect("build_data"), 3, &TK)
        .expect("protect");
    assert_eq!(whole, split, "build_data + protect equals encap");

    let back_whole = decap(&whole, &TK).expect("decap");
    let back_split = parse_data(&unprotect(&whole, &TK).expect("unprotect")).expect("parse_data");
    assert_eq!(back_whole, back_split, "unprotect + parse_data equals decap");
    assert_eq!(back_split, eth, "the split receive path recovers the frame");
}

#[test]
fn hardware_ccmp_plaintext_path_round_trips() {
    // A hardware-CCMP chip is handed `build_data` output and encrypts it
    // itself; on receive it delivers a decrypted MPDU that `parse_data` turns
    // back into ethernet. The plaintext MPDU must therefore be unencrypted, and
    // build/parse must be inverses without any software CCMP in the loop.
    let eth = ethernet_frame();
    let mpdu = build_data(&eth, OUR, BSSID, 9).expect("build_data");

    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
    assert_eq!(fc & FC_PROTECTED, 0, "the frame handed to the radio is not marked encrypted");
    const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];
    assert_eq!(&mpdu[24..30], &LLC_SNAP, "the body is plaintext LLC/SNAP, not ciphertext");

    let back = parse_data(&mpdu).expect("parse_data");
    assert_eq!(back, eth, "build_data and parse_data are inverses for a hardware-CCMP chip");
}

#[test]
fn parse_data_strips_the_ccmp_header_on_a_hardware_decrypted_frame() {
    // What the RTL8821CE actually delivers on receive: it decrypts the payload in
    // hardware but hands up the MPDU with the 802.11 header AND the 8-byte CCMP
    // header still present, and leaves the Protected bit set in the frame control.
    // parse_data must skip the CCMP header to find LLC/SNAP; before it did, every
    // hardware-decrypted frame, the DHCP reply included, failed the LLC/SNAP check
    // and was dropped, so no address ever bound over wlan.
    const FC_FROM_DS: u16 = 0x0200;
    const SRC: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x09];
    let fc = frame_control(TYPE_DATA, 0) | FC_FROM_DS | FC_PROTECTED;
    let payload = [0xde, 0xad, 0xbe, 0xef, 0x01, 0x02];
    let mut mpdu = Vec::new();
    mpdu.extend_from_slice(&fc.to_le_bytes());
    mpdu.extend_from_slice(&[0, 0]); // duration
    mpdu.extend_from_slice(&OUR); // addr1 = destination station (FromDS)
    mpdu.extend_from_slice(&BSSID); // addr2 = BSSID (transmitter)
    mpdu.extend_from_slice(&SRC); // addr3 = original source
    mpdu.extend_from_slice(&[0, 0]); // sequence control
    mpdu.extend_from_slice(&[0x01, 0x02, 0x00, 0x20, 0x03, 0x04, 0x05, 0x06]); // CCMP header
    mpdu.extend_from_slice(&[0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00]); // LLC/SNAP
    mpdu.extend_from_slice(&[0x08, 0x00]); // ethertype IPv4
    mpdu.extend_from_slice(&payload);

    let eth = parse_data(&mpdu).expect("hardware-decrypted protected frame parses");
    assert_eq!(&eth[0..6], &OUR, "destination is addr1 for FromDS");
    assert_eq!(&eth[6..12], &SRC, "source is addr3 for FromDS");
    assert_eq!(&eth[12..14], &[0x08, 0x00], "ethertype preserved");
    assert_eq!(&eth[14..], &payload, "payload preserved after skipping the CCMP header");
}

#[test]
fn parse_data_handles_a_qos_data_frame_from_the_ap() {
    // A real 802.11n/ac AP delivers QoS data (FromDS): a 26-byte header with a
    // QoS Control field, then LLC/SNAP and the payload. parse_data must size the
    // header from the frame to find the body, or every received frame is lost.
    const SRC: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x09];
    let fc = frame_control(TYPE_DATA, 0x08) | 0x0200; // QoS data, FromDS
    let mut mpdu = Vec::new();
    mpdu.extend_from_slice(&fc.to_le_bytes());
    mpdu.extend_from_slice(&[0, 0]); // duration
    mpdu.extend_from_slice(&OUR); // addr1 = receiver (station)
    mpdu.extend_from_slice(&BSSID); // addr2 = transmitter (AP)
    mpdu.extend_from_slice(&SRC); // addr3 = source
    mpdu.extend_from_slice(&[0, 0]); // sequence control
    mpdu.extend_from_slice(&[0x00, 0x00]); // QoS control (the 2 extra bytes)
    mpdu.extend_from_slice(&[0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00]); // LLC/SNAP
    mpdu.extend_from_slice(&[0x08, 0x00]); // ethertype IPv4
    mpdu.extend_from_slice(b"qos payload from the air");

    let eth = parse_data(&mpdu).expect("parse a QoS data frame");
    assert_eq!(&eth[0..6], &OUR, "destination is addr1 for FromDS");
    assert_eq!(&eth[6..12], &SRC, "source is addr3 for FromDS");
    assert_eq!(&eth[12..14], &[0x08, 0x00], "ethertype preserved");
    assert_eq!(&eth[14..], b"qos payload from the air", "payload found after the QoS header");
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
