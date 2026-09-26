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

//! The enrolled-secret proof the kernel mints for a local build: what the
//! prover makes, the verifier accepts, and nothing else.

use crate::crypto::hash::blake3_hash;
use crate::crypto::zk_kernel::constants::{DOM_MERKLE_LEAF, DOM_MERKLE_NODE};
use crate::crypto::zk_kernel::{prove_enrolled, verify_enrolled, EnrolledSecretProof, PedersenCommitment};

const SIBLINGS: [[u8; 32]; 8] = [[0x11; 32]; 8];

fn hash(parts: &[&[u8]]) -> [u8; 32] {
    blake3_hash(&parts.concat())
}

/// Leaf 0's root, folded as `attest::merkle::fold_root` folds it: every
/// direction is left, so the running node always goes first.
fn root_for(commitment: &[u8; 32]) -> [u8; 32] {
    let mut cur = hash(&[DOM_MERKLE_LEAF, commitment]);
    for s in &SIBLINGS {
        cur = hash(&[DOM_MERKLE_NODE, &cur, s]);
    }
    cur
}

fn proved(x: u8, ctx: &[u8]) -> (EnrolledSecretProof, [u8; 32]) {
    let (secret, blinding) = ([x; 32], [x ^ 0x5a; 32]);
    let root = root_for(&PedersenCommitment::commit(&secret, &blinding).commitment);
    let proof = prove_enrolled(&secret, &blinding, 0, &SIBLINGS, &root, ctx).expect("proof");
    (proof, root)
}

#[test]
fn a_proof_the_kernel_makes_verifies() {
    for x in [1u8, 7, 0x80, 0xff] {
        let (p, root) = proved(x, b"ctx");
        assert!(verify_enrolled(&p, &root, b"ctx"), "secret byte {x:#x}");
    }
}

#[test]
fn it_verifies_for_nothing_else() {
    let (p, root) = proved(7, b"ctx");
    assert!(!verify_enrolled(&p, &root, b"other ctx"));
    assert!(!verify_enrolled(&p, &[0x42; 32], b"ctx"));
    let mut z = p.clone();
    z.z_x[0] ^= 1;
    assert!(!verify_enrolled(&z, &root, b"ctx"));
    let (other, _) = proved(9, b"ctx");
    assert!(!verify_enrolled(&other, &root, b"ctx"));
}
