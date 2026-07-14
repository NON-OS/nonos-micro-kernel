// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proof for the WPA2-PSK 4-way handshake, supplicant side. A
//! simulated authenticator (AP) drives the real `Supplicant` through messages
//! 1 and 3, and the proof asserts: the pairwise key the supplicant derives
//! matches an independent PTK computation, message 2 carries the SNonce with a
//! MIC the AP accepts, message 3's group key is unwrapped correctly, the final
//! state is Connected with the right TK and GTK, and a tampered message 3 (bad
//! MIC or wrong ANonce) is rejected instead of trusted. No hardware: the whole
//! handshake is EAPOL frames and AES over the wire.

use crate::ccmp::aes::Aes128;
use crate::eapol::build::build_key_frame;
use crate::eapol::mic::verify_mic;
use crate::eapol::parse::{parse, KEY_INFO_MIC, KEY_INFO_PAIRWISE, KEY_INFO_VERSION2};
use crate::wpa::ptk::{pmk, ptk};
use crate::wpa::supplicant::{State, Supplicant};

const AA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01]; // AP
const SPA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02]; // station
const ANONCE: [u8; 32] = [0xA1; 32];
const SNONCE: [u8; 32] = [0x52; 32];
const GTK: [u8; 16] = [
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
];

// RFC 3394 AES key wrap, the AP's half of the group-key delivery. The
// supplicant only unwraps; the proof implements the wrap so a full round trip
// can be exercised against the real unwrap.
fn aes_wrap(kek: &[u8; 16], plain: &[u8]) -> Vec<u8> {
    assert!(plain.len() % 8 == 0 && !plain.is_empty());
    let n = plain.len() / 8;
    let aes = Aes128::new(kek);
    let mut a = [0xA6u8; 8];
    let mut r: Vec<[u8; 8]> = (0..n)
        .map(|i| {
            let mut b = [0u8; 8];
            b.copy_from_slice(&plain[i * 8..i * 8 + 8]);
            b
        })
        .collect();
    for j in 0..6u64 {
        for i in 0..n {
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(&a);
            block[8..].copy_from_slice(&r[i]);
            aes.encrypt_block(&mut block);
            let t = (n as u64) * j + (i as u64) + 1;
            a.copy_from_slice(&block[..8]);
            for k in 0..8 {
                a[k] ^= (t >> (56 - 8 * k)) as u8;
            }
            r[i].copy_from_slice(&block[8..]);
        }
    }
    let mut out = Vec::with_capacity((n + 1) * 8);
    out.extend_from_slice(&a);
    for block in &r {
        out.extend_from_slice(block);
    }
    out
}

// The GTK key-data KDE, AES-wrapped under the KEK, as message 3 carries it.
fn wrapped_gtk_kde(kek: &[u8; 16]) -> Vec<u8> {
    let mut kde = vec![0xDD, 22, 0x00, 0x0f, 0xac, 0x01, 0x01, 0x00];
    kde.extend_from_slice(&GTK); // total 24 bytes, a multiple of 8
    aes_wrap(kek, &kde)
}

fn message1() -> Vec<u8> {
    let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE; // ACK, no MIC
    let mut out = [0u8; 160];
    // Message 1 has no MIC; the dummy key silences build's MIC slot, which the
    // supplicant ignores for message 1.
    let n = build_key_frame(&mut out, info, &[0, 0, 0, 0, 0, 0, 0, 1], &ANONCE, &[], &[0u8; 16])
        .unwrap();
    out[..n].to_vec()
}

fn message3(kck: &[u8], kek: &[u8; 16]) -> Vec<u8> {
    let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE | KEY_INFO_MIC;
    let kd = wrapped_gtk_kde(kek);
    let mut out = [0u8; 160];
    let n = build_key_frame(&mut out, info, &[0, 0, 0, 0, 0, 0, 0, 2], &ANONCE, &kd, kck).unwrap();
    out[..n].to_vec()
}

fn expected_ptk() -> [u8; 48] {
    let k = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    ptk(&k, &AA, &SPA, &ANONCE, &SNONCE)
}

#[test]
fn full_handshake_reaches_connected_with_correct_keys() {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = Supplicant::new(key, AA, SPA, SNONCE);

    // Message 1 -> message 2.
    let out = sup.step(&message1());
    assert_eq!(sup.state(), State::PtkDerived);
    let m2 = out.reply.expect("message 2 is produced");
    let k2 = parse(&m2).expect("message 2 parses");
    assert_eq!(k2.nonce, SNONCE, "message 2 carries the station's SNonce");
    let pexp = expected_ptk();
    assert!(verify_mic(&pexp[0..16], &m2), "message 2 MIC verifies under the derived KCK");
    assert_eq!(sup.tk(), &pexp[32..48], "the derived TK matches an independent PTK");

    // Message 3 -> message 4, connected, keys installed.
    let out = sup.step(&message3(&pexp[0..16], pexp[16..32].try_into().unwrap()));
    assert_eq!(sup.state(), State::Connected);
    let m4 = out.reply.expect("message 4 is produced");
    assert!(verify_mic(&pexp[0..16], &m4), "message 4 MIC verifies");
    assert_eq!(sup.gtk(), &GTK, "the group key was unwrapped correctly");
}

#[test]
fn tampered_message3_mic_is_rejected() {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = Supplicant::new(key, AA, SPA, SNONCE);
    sup.step(&message1());
    let pexp = expected_ptk();
    let mut m3 = message3(&pexp[0..16], pexp[16..32].try_into().unwrap());
    m3[83] ^= 0xFF; // corrupt a MIC byte
    let out = sup.step(&m3);
    assert_eq!(sup.state(), State::Failed, "a bad MIC abandons the handshake");
    assert!(out.reply.is_none());
}

#[test]
fn wrong_anonce_in_message3_is_rejected() {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let mut sup = Supplicant::new(key, AA, SPA, SNONCE);
    sup.step(&message1());
    let pexp = expected_ptk();
    // Rebuild message 3 with a different ANonce but a valid MIC: the ANonce
    // check, not the MIC, must catch this replay/mismatch.
    let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE | KEY_INFO_MIC;
    let kd = wrapped_gtk_kde(pexp[16..32].try_into().unwrap());
    let mut out = [0u8; 160];
    let bad = [0xB2u8; 32];
    let n = build_key_frame(&mut out, info, &[0, 0, 0, 0, 0, 0, 0, 2], &bad, &kd, &pexp[0..16])
        .unwrap();
    let r = sup.step(&out[..n]);
    assert_eq!(sup.state(), State::Failed, "an ANonce mismatch abandons the handshake");
    assert!(r.reply.is_none());
}
