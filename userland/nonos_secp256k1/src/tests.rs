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

//! What this curve must produce, pinned.
//!
//! The vectors below are not invented. They are what the kernel's secp256k1
//! produced for these inputs, captured while both implementations were in the
//! tree and proved byte-identical over sixty-four key and message pairs by
//! `crypto_proofs::secp256k1_migration_tests`. Recording them here is what
//! survives the kernel copy's deletion: a wallet that signs differently after
//! any future change to this crate fails on the exact bytes rather than on a
//! property that happens to still hold.
//!
//! The generator vector is the independent anchor. It is SEC 2's G, which is
//! a published constant rather than anything this system decided, so if the
//! field or point arithmetic were wrong in a way that moved every result
//! together, that test still catches it.

use crate::{public_key_from_secret, recover_public_key, sign, verify};

fn secret(seed: u8) -> [u8; 32] {
    let mut sk = [0u8; 32];
    for (i, b) in sk.iter_mut().enumerate() {
        *b = seed.wrapping_mul(31).wrapping_add(i as u8).wrapping_add(1);
    }
    sk
}

fn message(seed: u8) -> [u8; 32] {
    let mut m = [0u8; 32];
    for (i, b) in m.iter_mut().enumerate() {
        *b = seed.wrapping_mul(17).wrapping_add((i as u8).wrapping_mul(3));
    }
    m
}

fn hex(bytes: &[u8]) -> alloc::string::String {
    use core::fmt::Write;
    let mut s = alloc::string::String::new();
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn compact(r: &[u8; 32], s: &[u8; 32]) -> [u8; 64] {
    let mut out = [0u8; 64];
    out[..32].copy_from_slice(r);
    out[32..].copy_from_slice(s);
    out
}

/// seed, public key, r, s, recovery id.
const VECTORS: [(u8, &str, &str, &str, u8); 5] = [
    (
        0,
        "0484bf7562262bbd6940085748f3be6afa52ae317155181ece31b66351ccffa4b08cc43d63b2859d469fee15f31c9edb5324266e6fd0407e87382d60fc4511acd8",
        "0889026a5c574ec9bdb93966bd94279a7e04bfaa2b69019eda4185a011bdb2d4",
        "41cbc09635d8f6bc709bf26f49d4a9fd37628cfcee6972e8d9754e019f845665",
        1,
    ),
    (
        1,
        "041f8a566c205633d029094747d2e18f44e05993dda7a5f88f496078205f656e59d2403ec0d3f55660b64477acbb7f84bb9a14914c177df864dccba0ac1937ef40",
        "1d28ea91f90da584f2d2f8ea2747901cd55e8174dced0645c6f52ab21d60ab89",
        "251385907f60fa425b39b51899fe79890e3e64528a83c2a5cce8ebf1ca8c52a9",
        0,
    ),
    (
        7,
        "04ddbb8dfcc87e0c382ca3e1ced1bb9e9b6e1d8c9ca92985addb69693712cccf798fb94553fad8c4d7462e93a3b5d6efab647ddc897b8887fe3398960308eae288",
        "2bee42e7c4f2134797841ee3f8b83508185cc191430c5ddff3d46a300d029310",
        "2a271ea7b9328e101ea92e49d98e135415ba8d464fb1097eca639927091e84b3",
        0,
    ),
    (
        42,
        "04adca63413798668311a1e8a524b4969cf206cec957920eb1196e9092bbfaed0d859eb21271006d3553282d72df79ae97d7195e4f897c167331247ecaaa922071",
        "02eb75682f47b0a678cdde25ffc7532840597b3a688e7f0bd8fb0ce00a5e4259",
        "11701a1d25673b512e909810d491347a1339f195f47ed5a62c8ff00ea3918258",
        1,
    ),
    (
        255,
        "044579cbe250148e0e67b2a38d906c7e0b8881c1d524bf4fdf86b25647e0c712506d6edd02d1dc91f0ee53fbcd638a928d51685e9b059718554cf4b4a302fb8a9c",
        "fda89be4199776da1534710593e7f978d6a1002bce921ae4b7d85cfad0b18693",
        "56dd375b143a439ba8995be18b87fb0eb119ea07ab6be612a77c9099a4988bd3",
        1,
    ),
];

#[test]
fn signatures_match_the_recorded_vectors() {
    for (seed, pk_hex, r_hex, s_hex, v) in VECTORS {
        let sk = secret(seed);
        let pk = public_key_from_secret(&sk).expect("public key");
        assert_eq!(hex(&pk), pk_hex, "seed {seed}: public key");

        let sig = sign(&sk, &message(seed)).expect("sign");
        assert_eq!(hex(&sig.r), r_hex, "seed {seed}: r");
        assert_eq!(hex(&sig.s), s_hex, "seed {seed}: s");
        assert_eq!(sig.recovery_id, v, "seed {seed}: recovery id");
    }
}

#[test]
fn the_generator_is_the_standard_one() {
    // SEC 2 G, the one value here that comes from outside this system.
    let mut one = [0u8; 32];
    one[31] = 1;
    let pk = public_key_from_secret(&one).expect("1 is a valid scalar");
    assert_eq!(pk[0], 0x04, "uncompressed form");
    assert_eq!(
        hex(&pk[1..33]),
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "generator x"
    );
    assert_eq!(
        hex(&pk[33..65]),
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        "generator y"
    );
}

#[test]
fn signing_is_deterministic() {
    let sk = secret(3);
    let msg = message(3);
    let first = sign(&sk, &msg).expect("sign");
    for _ in 0..8 {
        let again = sign(&sk, &msg).expect("sign");
        assert_eq!(first.r, again.r);
        assert_eq!(first.s, again.s);
        assert_eq!(first.recovery_id, again.recovery_id);
    }
}

#[test]
fn a_signature_verifies_and_recovers_its_own_key() {
    for seed in 0..48u8 {
        let sk = secret(seed);
        let msg = message(seed);
        let pk = public_key_from_secret(&sk).expect("public key");
        let sig = sign(&sk, &msg).expect("sign");
        assert!(verify(&pk, &msg, &compact(&sig.r, &sig.s)), "seed {seed}: own signature failed");
        assert_eq!(recover_public_key(&msg, &sig), Some(pk), "seed {seed}: recovery");
    }
}

#[test]
fn every_bit_of_the_signature_matters() {
    let sk = secret(7);
    let msg = message(7);
    let pk = public_key_from_secret(&sk).expect("public key");
    let sig = sign(&sk, &msg).expect("sign");
    let good = compact(&sig.r, &sig.s);
    for byte in 0..64 {
        for bit in 0..8 {
            let mut bad = good;
            bad[byte] ^= 1 << bit;
            assert!(!verify(&pk, &msg, &bad), "accepted a flip at byte {byte} bit {bit}");
        }
    }
}

#[test]
fn a_signature_is_bound_to_its_message() {
    let sk = secret(11);
    let pk = public_key_from_secret(&sk).expect("public key");
    let sig = sign(&sk, &message(11)).expect("sign");
    let bytes = compact(&sig.r, &sig.s);
    for other in 0..32u8 {
        if other == 11 {
            continue;
        }
        assert!(!verify(&pk, &message(other), &bytes), "verified message {other}");
    }
}

#[test]
fn a_signature_is_bound_to_its_key() {
    let msg = message(5);
    let sig = sign(&secret(5), &msg).expect("sign");
    let bytes = compact(&sig.r, &sig.s);
    for other in 0..32u8 {
        if other == 5 {
            continue;
        }
        let pk = public_key_from_secret(&secret(other)).expect("public key");
        assert!(!verify(&pk, &msg, &bytes), "verified under key {other}");
    }
}

#[test]
fn every_signature_is_low_s() {
    /*
     * Half the order. Above it a signature is malleable: negating s gives a
     * second valid signature over the same message, and on a chain that is a
     * second transaction hash for one transaction.
     */
    const HALF_N: [u8; 32] = [
        0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0x5D, 0x57, 0x6E, 0x73, 0x57, 0xA4, 0x50, 0x1D, 0xDF, 0xE9, 0x2F, 0x46, 0x68, 0x1B,
        0x20, 0xA0,
    ];
    for seed in 0..64u8 {
        let sig = sign(&secret(seed), &message(seed)).expect("sign");
        assert!(sig.s <= HALF_N, "seed {seed}: s above n/2");
    }
}

#[test]
fn a_zero_secret_is_refused() {
    // Not a scalar, and the one input that would otherwise sign as infinity.
    assert!(public_key_from_secret(&[0u8; 32]).is_none());
    assert!(sign(&[0u8; 32], &message(1)).is_none());
}

#[test]
fn a_secret_at_or_above_the_order_is_refused() {
    // n itself, and n with the low byte raised.
    const N: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
        0x41, 0x41,
    ];
    assert!(public_key_from_secret(&N).is_none(), "the order is not a valid secret");
    let mut over = N;
    over[31] = 0x42;
    assert!(public_key_from_secret(&over).is_none(), "above the order is not a valid secret");
    assert!(public_key_from_secret(&[0xFF; 32]).is_none(), "all ones is above the order");
}

#[test]
fn a_malformed_public_key_never_verifies() {
    let sk = secret(9);
    let msg = message(9);
    let sig = sign(&sk, &msg).expect("sign");
    let bytes = compact(&sig.r, &sig.s);
    let good = public_key_from_secret(&sk).expect("public key");

    let mut wrong_tag = good;
    wrong_tag[0] = 0x02;
    assert!(!verify(&wrong_tag, &msg, &bytes), "accepted a key with the wrong prefix");

    let mut off_curve = good;
    off_curve[64] ^= 0x01;
    assert!(!verify(&off_curve, &msg, &bytes), "accepted a point off the curve");

    assert!(!verify(&[0u8; 65], &msg, &bytes), "accepted an all-zero key");
    assert!(!verify(&[0xFF; 65], &msg, &bytes), "accepted an all-ones key");
}

#[test]
fn a_zero_signature_never_verifies() {
    let pk = public_key_from_secret(&secret(4)).expect("public key");
    assert!(!verify(&pk, &message(4), &[0u8; 64]), "r and s of zero must be refused");
    let mut r_zero = [0u8; 64];
    r_zero[63] = 1;
    assert!(!verify(&pk, &message(4), &r_zero), "r of zero must be refused");
}
