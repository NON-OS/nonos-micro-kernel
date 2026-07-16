// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the EAPOL-Key handshake message layer. The parse is checked
//! structurally; the MIC is checked by round-trip and tamper. The MIC rests on
//! the RFC-verified HMAC-SHA1, so a correct round-trip plus a rejected tamper
//! and a rejected wrong key pin the verify logic exactly.

use crate::eapol::build::build_key_frame;
use crate::eapol::mic::{compute_mic, verify_mic};
use crate::eapol::parse::{
    parse, EAPOL_TYPE_KEY, HEADER_LEN, KEY_INFO_MIC, KEY_INFO_PAIRWISE, MIC_LEN, MIC_OFFSET,
};
use crate::wpa::ptk::{pmk, ptk};

fn build_frame(key_info: u16, nonce: &[u8; 32], key_data: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; HEADER_LEN + key_data.len()];
    f[0] = 2;
    f[1] = EAPOL_TYPE_KEY;
    let plen = (HEADER_LEN - 4 + key_data.len()) as u16;
    f[2..4].copy_from_slice(&plen.to_be_bytes());
    f[4] = 2;
    f[5..7].copy_from_slice(&key_info.to_be_bytes());
    f[17..49].copy_from_slice(nonce);
    f[97..99].copy_from_slice(&(key_data.len() as u16).to_be_bytes());
    f[HEADER_LEN..].copy_from_slice(key_data);
    f
}

#[test]
fn parse_reads_the_key_fields() {
    let nonce = [0x5au8; 32];
    let f = build_frame(KEY_INFO_MIC | KEY_INFO_PAIRWISE, &nonce, &[0xaa, 0xbb]);
    let k = parse(&f).unwrap();
    assert_eq!(k.key_info & KEY_INFO_MIC, KEY_INFO_MIC);
    assert_eq!(k.key_info & KEY_INFO_PAIRWISE, KEY_INFO_PAIRWISE);
    assert_eq!(k.nonce, nonce);
    assert_eq!(k.key_data, &[0xaa, 0xbb]);
}

#[test]
fn parse_rejects_short_and_non_key_frames() {
    assert!(parse(&[0u8; HEADER_LEN - 1]).is_none(), "too short");
    let mut f = vec![0u8; HEADER_LEN];
    f[1] = 0x00;
    assert!(parse(&f).is_none(), "not a key frame");
    // key-data length running past the buffer is rejected
    let mut f = vec![0u8; HEADER_LEN];
    f[1] = EAPOL_TYPE_KEY;
    f[97..99].copy_from_slice(&100u16.to_be_bytes());
    assert!(parse(&f).is_none(), "key-data length overruns");
}

#[test]
fn mic_round_trips_and_rejects_tampering_and_wrong_key() {
    let kck = [0x11u8; 16];
    let nonce = [0x22u8; 32];
    let mut f = build_frame(KEY_INFO_MIC, &nonce, &[1, 2, 3, 4]);
    let mic = compute_mic(&kck, &f);
    f[MIC_OFFSET..MIC_OFFSET + MIC_LEN].copy_from_slice(&mic);
    assert!(verify_mic(&kck, &f), "a correct MIC verifies");

    f[20] ^= 0x01;
    assert!(!verify_mic(&kck, &f), "a tampered frame is rejected");
    f[20] ^= 0x01;

    assert!(!verify_mic(&[0u8; 16], &f), "the wrong KCK is rejected");
}

#[test]
fn mic_ignores_trailing_mpdu_bytes_after_the_key_data() {
    // A received EAPOL-Key frame arrives inside an 802.11 MPDU and can carry a
    // trailing FCS or padding past its key data. The MIC covers only the key
    // frame, so those bytes must not enter the hash. This is the four-way
    // failure seen on hardware: message 3 carries a group-key KDE, and the four
    // FCS bytes behind it made the otherwise-correct MIC verify fail.
    let kck = [0x11u8; 16];
    let nonce = [0x22u8; 32];
    let mut f = build_frame(KEY_INFO_MIC, &nonce, &[1, 2, 3, 4, 5, 6, 7, 8]);
    let mic = compute_mic(&kck, &f);
    f[MIC_OFFSET..MIC_OFFSET + MIC_LEN].copy_from_slice(&mic);
    assert!(verify_mic(&kck, &f), "the exact key frame verifies");

    f.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
    assert!(verify_mic(&kck, &f), "a trailing FCS does not break the MIC");
    *f.last_mut().unwrap() ^= 0xff;
    assert!(verify_mic(&kck, &f), "trailer contents are outside the MIC input");
}

#[test]
fn a_built_frame_verifies_parses_and_rejects_the_wrong_key() {
    let kck = [0x33u8; 16];
    let replay = [0, 0, 0, 0, 0, 0, 0, 2];
    let nonce = [0x44u8; 32];
    let key_data = [0xde, 0xad, 0xbe, 0xef];
    let mut frame = [0u8; 128];
    let n = build_key_frame(
        &mut frame,
        KEY_INFO_MIC | KEY_INFO_PAIRWISE,
        &replay,
        &nonce,
        &key_data,
        &kck,
    )
    .unwrap();
    assert!(verify_mic(&kck, &frame[..n]), "the built MIC verifies under the same KCK");
    let k = parse(&frame[..n]).unwrap();
    assert_eq!(k.nonce, nonce);
    assert_eq!(k.replay_counter, replay);
    assert_eq!(k.key_data, &key_data);
    assert!(!verify_mic(&[0u8; 16], &frame[..n]), "the wrong KCK does not verify");
}

#[test]
fn the_supplicant_message_two_path_composes_end_to_end() {
    // The full message-two crypto: derive the PTK from the PMK and the
    // handshake nonces, build message two carrying the supplicant nonce, and
    // confirm its MIC verifies under the KCK taken from that PTK.
    let key = pmk(b"password", b"IEEE");
    let aa = [0, 0, 0, 0, 0, 1];
    let spa = [0, 0, 0, 0, 0, 2];
    let anonce = [0xaau8; 32];
    let snonce = [0xbbu8; 32];
    let p = ptk(&key, &aa, &spa, &anonce, &snonce);
    let kck = &p[..16];
    let mut msg2 = [0u8; 128];
    let n = build_key_frame(
        &mut msg2,
        KEY_INFO_MIC | KEY_INFO_PAIRWISE,
        &[0, 0, 0, 0, 0, 0, 0, 1],
        &snonce,
        &[],
        kck,
    )
    .unwrap();
    assert!(verify_mic(kck, &msg2[..n]), "message two verifies with the derived KCK");
    assert_eq!(parse(&msg2[..n]).unwrap().nonce, snonce);
}
