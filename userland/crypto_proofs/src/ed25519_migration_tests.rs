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

//! The kernel's ed25519 and the userland one, side by side.
//!
//! The mixnet's signing and verification are leaving ring 0; the boot chain's
//! verification stays, because it is what decides whether a capsule loads at
//! all. Before the syscalls go, both implementations run over the same inputs
//! and must agree on every byte.

use crate::crypto::asymmetric::ed25519 as kernel;
use nonos_ed25519 as userland;

fn seed(n: u8) -> [u8; 32] {
    let mut s = [0u8; 32];
    for (i, b) in s.iter_mut().enumerate() {
        *b = n.wrapping_mul(13).wrapping_add(i as u8).wrapping_add(1);
    }
    s
}

fn message(n: u8) -> alloc::vec::Vec<u8> {
    (0..=n).map(|i| i.wrapping_mul(7).wrapping_add(n)).collect()
}

#[test]
fn public_keys_match() {
    for n in 0..64u8 {
        assert_eq!(
            kernel::pubkey_from_secret(&seed(n)),
            userland::pubkey_from_secret(&seed(n)),
            "seed {n}"
        );
    }
}

#[test]
fn signatures_are_byte_identical() {
    for n in 0..64u8 {
        let k = kernel::KeyPair::from_seed(seed(n));
        let u = userland::KeyPair::from_seed(seed(n));
        let msg = message(n);
        let ks = kernel::sign(&k, &msg);
        let us = userland::sign(&u, &msg);
        assert_eq!(ks.to_bytes(), us.to_bytes(), "seed {n}: signature differs");
    }
}

#[test]
fn each_verifies_the_other() {
    for n in 0..32u8 {
        let k = kernel::KeyPair::from_seed(seed(n));
        let u = userland::KeyPair::from_seed(seed(n));
        let msg = message(n);
        let ks = kernel::sign(&k, &msg);
        let us = userland::sign(&u, &msg);
        assert!(userland::verify(&u.public, &msg, &userland::Signature::from_bytes(&ks.to_bytes())));
        assert!(kernel::verify(&k.public, &msg, &kernel::Signature::from_bytes(&us.to_bytes())));
    }
}

#[test]
fn both_reject_a_tampered_signature() {
    let k = kernel::KeyPair::from_seed(seed(9));
    let u = userland::KeyPair::from_seed(seed(9));
    let msg = message(9);
    let sig = userland::sign(&u, &msg).to_bytes();
    for i in 0..64 {
        let mut bad = sig;
        bad[i] ^= 0x01;
        assert!(!userland::verify(&u.public, &msg, &userland::Signature::from_bytes(&bad)), "userland at {i}");
        assert!(!kernel::verify(&k.public, &msg, &kernel::Signature::from_bytes(&bad)), "kernel at {i}");
    }
}

#[test]
fn both_reject_a_changed_message() {
    let u = userland::KeyPair::from_seed(seed(4));
    let k = kernel::KeyPair::from_seed(seed(4));
    let sig = userland::sign(&u, &message(4)).to_bytes();
    for other in 0..16u8 {
        if other == 4 {
            continue;
        }
        let m = message(other);
        assert!(!userland::verify(&u.public, &m, &userland::Signature::from_bytes(&sig)));
        assert!(!kernel::verify(&k.public, &m, &kernel::Signature::from_bytes(&sig)));
    }
}
