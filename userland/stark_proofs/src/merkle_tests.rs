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

use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::merkle::{verify_path, verify_path_ext, MerkleTree};

extern crate alloc;
use alloc::vec::Vec;

// A Merkle commitment is the binding layer a STARK verifier trusts: the prover
// commits to a column of evaluations, then opens a few positions. Soundness of
// the whole proof rests on two properties, both checked here on the real code:
// an honest opening always verifies, and no tampering of the leaf, the path, or
// the root is ever accepted.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn leaves(n: usize, seed: u64) -> Vec<Fp> {
    let mut s = seed | 1;
    (0..n).map(|_| Fp::from_u64(xorshift(&mut s))).collect()
}

#[test]
fn honest_openings_always_verify() {
    let mut seed = 0x1234_5678_9abc_def0u64;
    for &n in &[1usize, 2, 3, 4, 5, 8, 16, 17, 64, 100, 256] {
        let ls = leaves(n, seed);
        let tree = MerkleTree::commit(&ls);
        let root = tree.root();
        for (i, &leaf) in ls.iter().enumerate() {
            let path = tree.open(i);
            assert!(verify_path(&root, i, leaf, &path), "honest opening at {i} of {n} failed");
        }
        seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    }
}

#[test]
fn a_tampered_leaf_is_rejected() {
    let ls = leaves(64, 42);
    let tree = MerkleTree::commit(&ls);
    let root = tree.root();
    for i in [0usize, 1, 7, 31, 63] {
        let path = tree.open(i);
        // The real leaf verifies; a different value at the same position does not.
        assert!(verify_path(&root, i, ls[i], &path));
        let wrong = ls[i] + Fp::ONE;
        assert!(!verify_path(&root, i, wrong, &path), "a tampered leaf verified");
    }
}

#[test]
fn a_tampered_path_or_root_is_rejected() {
    let ls = leaves(32, 7);
    let tree = MerkleTree::commit(&ls);
    let root = tree.root();
    let i = 5usize;
    let leaf = ls[i];
    let path = tree.open(i);
    assert!(verify_path(&root, i, leaf, &path));

    // Flip a bit in the first sibling.
    if !path.is_empty() {
        let mut bad = path.clone();
        bad[0][0] ^= 0x01;
        assert!(!verify_path(&root, i, leaf, &bad), "a tampered path verified");
    }
    // Flip a bit in the root.
    let mut bad_root = root;
    bad_root[0] ^= 0x01;
    assert!(!verify_path(&bad_root, i, leaf, &path), "a tampered root verified");
    // Wrong position for the same leaf and path.
    assert!(!verify_path(&root, i + 1, leaf, &path), "a wrong index verified");
}

#[test]
fn a_path_of_the_wrong_length_is_rejected() {
    // The verifier binds the path length to the leaf's depth through its final
    // `idx == 0` check. A truncated path forges a shallower position; an
    // over-long path overshoots the root. Both must fail, or a prover could open
    // to a position it never committed.
    let ls = leaves(64, 99);
    let tree = MerkleTree::commit(&ls);
    let root = tree.root();
    for i in [0usize, 1, 9, 40, 63] {
        let leaf = ls[i];
        let path = tree.open(i);
        assert!(verify_path(&root, i, leaf, &path));

        // Drop the topmost sibling: the recomputation stops below the root.
        let mut short = path.clone();
        short.pop();
        assert!(!verify_path(&root, i, leaf, &short), "a truncated path verified");

        // Append a sibling: the recomputation runs one level past the root.
        let mut long = path.clone();
        long.push([0xABu8; 32]);
        assert!(!verify_path(&root, i, leaf, &long), "an over-long path verified");
    }
}

#[test]
fn distinct_leaf_sets_give_distinct_roots() {
    // Binding: changing any leaf changes the root (collision would break BLAKE3).
    let a = MerkleTree::commit(&leaves(64, 1)).root();
    let b = MerkleTree::commit(&leaves(64, 2)).root();
    assert_ne!(a, b);
    let mut ls = leaves(64, 1);
    ls[20] = ls[20] + Fp::ONE;
    let c = MerkleTree::commit(&ls).root();
    assert_ne!(a, c, "a single changed leaf must change the root");
}

// The extension-field commitment is what the folded FRI layers use: same tree,
// same node hashing, an Fp2 leaf. Honest openings verify and tampering fails,
// exactly as for base leaves.

#[test]
fn extension_openings_verify_and_tampering_fails() {
    let mut s = 0x5eed_1234u64 | 1;
    let n = 32usize;
    let leaves: Vec<Fp2> = (0..n)
        .map(|_| Fp2::new(Fp::from_u64(xorshift(&mut s)), Fp::from_u64(xorshift(&mut s))))
        .collect();
    let tree = MerkleTree::commit_ext(&leaves);
    let root = tree.root();

    for (i, &leaf) in leaves.iter().enumerate() {
        let path = tree.open(i);
        assert!(verify_path_ext(&root, i, leaf, &path), "honest ext opening rejected at {i}");
        // A tampered leaf must fail.
        let bad = Fp2::new(leaf.c0 + Fp::ONE, leaf.c1);
        assert!(!verify_path_ext(&root, i, bad, &path), "tampered ext leaf accepted at {i}");
        // The wrong index must fail.
        assert!(!verify_path_ext(&root, i ^ 1, leaf, &path), "wrong index accepted at {i}");
    }
}

#[test]
fn base_and_extension_leaves_are_domain_separated() {
    // A base leaf and an extension leaf embedding the same value must commit to
    // different roots, so a base-layer proof can never be replayed as an ext layer.
    let v = Fp::from_u64(0x1234_5678);
    let base = MerkleTree::commit(&[v]);
    let ext = MerkleTree::commit_ext(&[Fp2::from_base(v)]);
    assert_ne!(base.root(), ext.root());
}
