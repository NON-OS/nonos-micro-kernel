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

//! A verify with nothing before it.
//!
//! Its own binary on purpose: an integration test is a fresh process, so no
//! sign or key derivation can have run first. The library once waited on a
//! base-point table that only those two built, and a process whose first
//! curve operation was a verify never returned.

use nonos_ed25519::{verify, Signature};

fn hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

/// RFC 8032, section 7.1, test 1: an empty message.
#[test]
fn verify_is_the_first_thing_this_process_does() {
    let pk: [u8; 32] =
        hex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a").try_into().unwrap();
    let sig: [u8; 64] = hex(concat!(
        "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33b",
        "acc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
    ))
    .try_into()
    .unwrap();
    assert!(verify(&pk, b"", &Signature::from_bytes(&sig)));
    let mut bad = sig;
    bad[0] ^= 1;
    assert!(!verify(&pk, b"", &Signature::from_bytes(&bad)));
}
