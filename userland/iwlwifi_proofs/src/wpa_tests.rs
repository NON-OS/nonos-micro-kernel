// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for the WPA2 key derivation. Each primitive is checked
//! against a published vector: SHA-1 (RFC 3174), HMAC-SHA1 (RFC 2202), PBKDF2
//! (RFC 6070) and the pairwise master key (IEEE 802.11i Annex H.4). A correct
//! PMK, which runs 4096 PBKDF2 rounds over HMAC-SHA1 over SHA-1, exercises the
//! whole stack end to end.

use crate::wpa::hmac::hmac_sha1;
use crate::wpa::pbkdf2::pbkdf2_sha1;
use crate::wpa::ptk::{pmk, ptk};
use crate::wpa::sha1::sha1;

fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn sha1_matches_rfc3174_vectors() {
    assert_eq!(sha1(b"abc")[..], hex("a9993e364706816aba3e25717850c26c9cd0d89d")[..]);
    assert_eq!(sha1(b"")[..], hex("da39a3ee5e6b4b0d3255bfef95601890afd80709")[..]);
    assert_eq!(
        sha1(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")[..],
        hex("84983e441c3bd26ebaae4aa1f95129e5e54670f1")[..]
    );
}

#[test]
fn hmac_sha1_matches_rfc2202_vectors() {
    assert_eq!(
        hmac_sha1(&[0x0b; 20], b"Hi There")[..],
        hex("b617318655057264e28bc0b6fb378c8ef146be00")[..]
    );
    assert_eq!(
        hmac_sha1(b"Jefe", b"what do ya want for nothing?")[..],
        hex("effcdf6ae5eb2fa2d27416d5f184df9c259a7c79")[..]
    );
}

#[test]
fn pbkdf2_matches_rfc6070_vectors() {
    let mut o = [0u8; 20];
    pbkdf2_sha1(b"password", b"salt", 1, &mut o);
    assert_eq!(o[..], hex("0c60c80f961f0e71f3a9b524af6012062fe037a6")[..]);
    pbkdf2_sha1(b"password", b"salt", 4096, &mut o);
    assert_eq!(o[..], hex("4b007901b765489abead49d926f721d065a429c1")[..]);
}

#[test]
fn pmk_matches_ieee_80211i_vectors() {
    assert_eq!(
        pmk(b"password", b"IEEE")[..],
        hex("f42c6fc52df0ebef9ebb4b90b38a5f902e83fe1b135a70e23aed762e9710a12e")[..]
    );
    assert_eq!(
        pmk(b"ThisIsAPassword", b"ThisIsASSID")[..],
        hex("0dc0d6eb90555ed6419756b9a15ec3e3209b63df707dd508d14581f8982721af")[..]
    );
}

#[test]
fn ptk_is_deterministic_and_independent_of_handshake_order() {
    let key = pmk(b"password", b"IEEE");
    let aa = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    let spa = [0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb];
    let anonce = [0x11u8; 32];
    let snonce = [0x22u8; 32];
    let p1 = ptk(&key, &aa, &spa, &anonce, &snonce);
    // both peers derive the same key regardless of who is the authenticator
    let p2 = ptk(&key, &spa, &aa, &snonce, &anonce);
    assert_eq!(p1, p2, "the PTK does not depend on address or nonce order");
    assert_eq!(p1.len(), 48);
    assert_eq!(ptk(&key, &aa, &spa, &anonce, &snonce), p1, "deterministic");
}
