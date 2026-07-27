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

//! Proven against the RFC 8439 known-answer vectors: the ChaCha20 block
//! function (Section 2.3.2), the Poly1305 authenticator (Section 2.5.2), and
//! the full AEAD_CHACHA20_POLY1305 seal (Section 2.8.2). Then the properties
//! the keyring relies on: a round-trip recovers the seed, and any wrong key,
//! tampered ciphertext, wrong nonce or wrong associated data is rejected.

use crate::chacha20;
use crate::poly1305::Poly1305;
use crate::seal::{open, seal, SealError, SealState, TAG_LEN};

fn hexb<const N: usize>(s: &str) -> [u8; N] {
    let bytes = s.as_bytes();
    let mut out = [0u8; N];
    let mut i = 0;
    while i < N {
        let hi = (bytes[i * 2] as char).to_digit(16).unwrap() as u8;
        let lo = (bytes[i * 2 + 1] as char).to_digit(16).unwrap() as u8;
        out[i] = (hi << 4) | lo;
        i += 1;
    }
    out
}

#[test]
fn chacha20_block_rfc_2_3_2() {
    let key: [u8; 32] = hexb("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let nonce: [u8; 12] = hexb("000000090000004a00000000");
    let block = chacha20::block(&key, &nonce, 1);
    let expected: [u8; 64] = hexb(
        "10f1e7e4d13b5915500fdd1fa32071c4c7d1f4c733c068030422aa9ac3d46c4e\
         d2826446079faa0914c2d705d98b02a2b5129cd1de164eb9cbd083e8a2503c4e",
    );
    assert_eq!(block, expected);
}

#[test]
fn poly1305_rfc_2_5_2() {
    let key: [u8; 32] = hexb("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
    let msg = b"Cryptographic Forum Research Group";
    let mut mac = Poly1305::new(&key);
    mac.update(msg);
    let tag = mac.finalize();
    let expected: [u8; 16] = hexb("a8061dc1305136c6c22b8baf0c0127a9");
    assert_eq!(tag, expected);
}

#[test]
fn aead_seal_rfc_2_8_2() {
    let key: [u8; 32] = hexb("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f");
    let nonce: [u8; 12] = hexb("070000004041424344454647");
    let aad: [u8; 12] = hexb("50515253c0c1c2c3c4c5c6c7");
    let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you \
only one tip for the future, sunscreen would be it.";

    let mut out = [0u8; 114 + TAG_LEN];
    let n = seal(&key, &nonce, &aad, plaintext, &mut out).unwrap();
    assert_eq!(n, 114 + TAG_LEN);

    let expected_ct: [u8; 114] = hexb(
        "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d6\
         3dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b36\
         92ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc\
         3ff4def08e4b7a9de576d26586cec64b6116",
    );
    let expected_tag: [u8; 16] = hexb("1ae10b594f09e26a7e902ecbd0600691");
    assert_eq!(&out[..114], &expected_ct[..]);
    assert_eq!(&out[114..], &expected_tag[..]);
}

#[test]
fn round_trip_recovers_the_seed() {
    let key = [0x42u8; 32];
    let nonce = [0x07u8; 12];
    let seed = [0x9au8; 64];
    let aad = b"nonos.wallet.seed.v1";

    let mut sealed = [0u8; 64 + TAG_LEN];
    seal(&key, &nonce, aad, &seed, &mut sealed).unwrap();
    // Ciphertext is not the plaintext.
    assert_ne!(&sealed[..64], &seed[..]);

    let mut recovered = [0u8; 64];
    let n = open(&key, &nonce, aad, &sealed, &mut recovered).unwrap();
    assert_eq!(n, 64);
    assert_eq!(recovered, seed);
}

#[test]
fn wrong_key_is_rejected() {
    let key = [0x42u8; 32];
    let mut wrong = key;
    wrong[0] ^= 1;
    let nonce = [0x07u8; 12];
    let seed = [0x9au8; 64];
    let aad = b"nonos.wallet.seed.v1";

    let mut sealed = [0u8; 64 + TAG_LEN];
    seal(&key, &nonce, aad, &seed, &mut sealed).unwrap();

    let mut out = [0u8; 64];
    assert_eq!(open(&wrong, &nonce, aad, &sealed, &mut out), Err(SealError::AuthFailed));
    // Nothing usable was written on the failure path.
    assert_eq!(out, [0u8; 64]);
}

#[test]
fn tamper_and_wrong_aad_are_rejected() {
    let key = [0x11u8; 32];
    let nonce = [0x22u8; 12];
    let seed = [0x33u8; 64];

    let mut sealed = [0u8; 64 + TAG_LEN];
    seal(&key, &nonce, b"aad-a", &seed, &mut sealed).unwrap();

    // Flip one ciphertext bit.
    let mut tampered = sealed;
    tampered[3] ^= 0x80;
    let mut out = [0u8; 64];
    assert_eq!(open(&key, &nonce, b"aad-a", &tampered, &mut out), Err(SealError::AuthFailed));

    // Different associated data over the untouched blob.
    assert_eq!(open(&key, &nonce, b"aad-b", &sealed, &mut out), Err(SealError::AuthFailed));

    // Wrong nonce.
    let other_nonce = [0x23u8; 12];
    assert_eq!(open(&key, &other_nonce, b"aad-a", &sealed, &mut out), Err(SealError::AuthFailed));
}

#[test]
fn seal_state_never_repeats_a_nonce_and_round_trips() {
    // Persist and reload: continuing from the saved counter never reuses a
    // nonce, and each blob opens at its own counter.
    let key = [0x5cu8; 32];
    let mut st = SealState::new(key, 0);
    let seed_a = [0xa1u8; 64];
    let seed_b = [0xb2u8; 64];

    let mut blob_a = [0u8; 64 + TAG_LEN];
    let mut blob_b = [0u8; 64 + TAG_LEN];
    let (ctr_a, _) = st.seal_next(b"", &seed_a, &mut blob_a).unwrap();
    let (ctr_b, _) = st.seal_next(b"", &seed_b, &mut blob_b).unwrap();
    assert_ne!(ctr_a, ctr_b, "counters, hence nonces, must differ");
    assert_ne!(&blob_a[..], &blob_b[..]);

    // Simulate reboot: rebuild the sequencer from the persisted counter.
    let saved = st.counter();
    let reloaded = SealState::new(key, saved);
    let mut out = [0u8; 64];
    assert_eq!(reloaded.open_at(ctr_a, b"", &blob_a, &mut out).unwrap(), 64);
    assert_eq!(out, seed_a);
    assert_eq!(reloaded.open_at(ctr_b, b"", &blob_b, &mut out).unwrap(), 64);
    assert_eq!(out, seed_b);

    // A fresh seal after reload uses a counter past the saved point.
    let mut st2 = SealState::new(key, saved);
    let (ctr_c, _) = st2.seal_next(b"", &seed_a, &mut blob_a).unwrap();
    assert!(ctr_c >= saved, "post-reboot nonce must not replay an earlier one");
}
