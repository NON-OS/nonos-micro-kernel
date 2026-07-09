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

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::poseidon_merkle::{verify_path, PoseidonMerkleTree};

extern crate alloc;
use alloc::vec::Vec;

// The Poseidon Merkle commitment is the recursion-friendly one: its node hash is
// a permutation of field elements, so a path check is a fixed sequence of field
// operations and can be proven inside another STARK. These checks establish the
// same soundness the BLAKE3 tree has, on the real Poseidon compression: honest
// openings verify, and nothing tampered ever does.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn leaves(n: usize, seed: u64) -> Vec<[Fp; RATE]> {
    let mut s = seed | 1;
    (0..n)
        .map(|_| {
            let mut d = [Fp::ZERO; RATE];
            for c in d.iter_mut() {
                *c = Fp::from_u64(xorshift(&mut s));
            }
            d
        })
        .collect()
}

fn hasher() -> Poseidon {
    Poseidon::new(5, [Fp::ZERO; RATE])
}

#[test]
fn the_compression_diffuses() {
    // Changing one input lane changes every output lane: the node hash mixes.
    let h = hasher();
    let a = h.compress(
        &[Fp::from_u64(1), Fp::from_u64(2), Fp::from_u64(3), Fp::from_u64(4)],
        &[Fp::ZERO; RATE],
    );
    let b = h.compress(
        &[Fp::from_u64(1), Fp::from_u64(2), Fp::from_u64(3), Fp::from_u64(5)],
        &[Fp::ZERO; RATE],
    );
    for i in 0..RATE {
        assert_ne!(a[i], b[i], "compression lane {i} did not diffuse");
    }
}

#[test]
fn honest_openings_always_verify() {
    let h = hasher();
    for &n in &[1usize, 2, 3, 5, 8, 16, 100] {
        let ls = leaves(n, 0x100 + n as u64);
        let tree = PoseidonMerkleTree::commit(&h, &ls);
        let root = tree.root();
        for (i, &leaf) in ls.iter().enumerate() {
            let path = tree.open(i);
            assert!(verify_path(&h, &root, i, leaf, &path), "honest opening at {i} of {n} failed");
        }
    }
}

#[test]
fn tampering_is_rejected() {
    let h = hasher();
    let ls = leaves(32, 7);
    let tree = PoseidonMerkleTree::commit(&h, &ls);
    let root = tree.root();
    let i = 5usize;
    let path = tree.open(i);
    assert!(verify_path(&h, &root, i, ls[i], &path));

    // Tampered leaf.
    let mut wrong = ls[i];
    wrong[0] = wrong[0] + Fp::ONE;
    assert!(!verify_path(&h, &root, i, wrong, &path), "a tampered leaf verified");
    // Tampered sibling.
    let mut bad = path.clone();
    bad[0][0] = bad[0][0] + Fp::ONE;
    assert!(!verify_path(&h, &root, i, ls[i], &bad), "a tampered path verified");
    // Tampered root.
    let mut bad_root = root;
    bad_root[0] = bad_root[0] + Fp::ONE;
    assert!(!verify_path(&h, &bad_root, i, ls[i], &path), "a tampered root verified");
    // Wrong position.
    assert!(!verify_path(&h, &root, i + 1, ls[i], &path), "a wrong index verified");
    // Wrong path length.
    let mut short = path.clone();
    short.pop();
    assert!(!verify_path(&h, &root, i, ls[i], &short), "a truncated path verified");
}

#[test]
fn distinct_leaf_sets_give_distinct_roots() {
    let h = hasher();
    let a = PoseidonMerkleTree::commit(&h, &leaves(64, 1)).root();
    let b = PoseidonMerkleTree::commit(&h, &leaves(64, 2)).root();
    assert_ne!(a, b);
    let mut ls = leaves(64, 1);
    ls[20][2] = ls[20][2] + Fp::ONE;
    let c = PoseidonMerkleTree::commit(&h, &ls).root();
    assert_ne!(a, c, "a single changed leaf must change the root");
}
