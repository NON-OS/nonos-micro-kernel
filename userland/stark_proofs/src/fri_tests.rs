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
use crate::crypto::stark::fri::{
    fold_ext, fold_first, fold_layer, fri_prove, fri_verify, root_of_unity,
};
use crate::crypto::stark::poly::eval;

extern crate alloc;
use alloc::vec::Vec;

// FRI is the low-degree test at the heart of a STARK: it convinces a verifier
// that a committed codeword is close to a low-degree polynomial, with no trusted
// setup and hash-only cryptography. Completeness (honest low-degree codewords
// pass) and soundness (high-degree codewords and forged proofs fail) are both
// checked here against the real prover and verifier.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Evaluate a random polynomial of degree below `d` over the size-`2^log_n`
/// domain `shift * {omega^i}`, giving a genuine low-degree codeword.
fn low_degree_codeword(log_n: u32, d: usize, shift: Fp, seed: u64) -> Vec<Fp> {
    let n = 1usize << log_n;
    let omega = root_of_unity(log_n);
    let mut s = seed | 1;
    let coeffs: Vec<Fp> = (0..d).map(|_| Fp::from_u64(xorshift(&mut s))).collect();
    let mut x = shift;
    let mut codeword = Vec::with_capacity(n);
    for _ in 0..n {
        codeword.push(eval(&coeffs, x));
        x = x * omega;
    }
    codeword
}

fn random_codeword(log_n: u32, seed: u64) -> Vec<Fp> {
    let n = 1usize << log_n;
    let mut s = seed | 1;
    (0..n).map(|_| Fp::from_u64(xorshift(&mut s))).collect()
}

#[test]
fn the_domain_generator_has_the_right_order() {
    // omega must have order exactly 2^log_n: omega^n == 1 and omega^(n/2) == -1,
    // which is what makes the domain closed under negation for folding.
    let minus_one = Fp::ZERO - Fp::ONE;
    for log_n in [2u32, 3, 5, 8, 10, 16] {
        let n = 1u64 << log_n;
        let omega = root_of_unity(log_n);
        assert_eq!(omega.pow(n), Fp::ONE, "omega^n must be one");
        assert_eq!(omega.pow(n / 2), minus_one, "omega^(n/2) must be minus one");
    }
}

#[test]
fn an_honest_low_degree_codeword_verifies() {
    let (log_n, log_blowup, degree, queries) = (5u32, 2u32, 8usize, 40usize);
    let codeword = low_degree_codeword(log_n, degree, Fp::ONE, 0xABCD);
    let proof = fri_prove(&codeword, Fp::ONE, log_blowup, queries);
    assert!(
        fri_verify(&proof, Fp::ONE, log_n, log_blowup, queries),
        "an honest proof was rejected"
    );
}

#[test]
fn an_honest_codeword_on_a_coset_verifies() {
    // The STARK runs FRI on an LDE coset, not the raw subgroup. Folding must hold
    // there too: evaluate a low-degree polynomial on shift * {omega^i} and prove.
    let (log_n, log_blowup, degree, queries) = (6u32, 2u32, 8usize, 40usize);
    let shift = Fp::from_u64(7);
    let codeword = low_degree_codeword(log_n, degree, shift, 0xC05E7);
    let proof = fri_prove(&codeword, shift, log_blowup, queries);
    assert!(
        fri_verify(&proof, shift, log_n, log_blowup, queries),
        "an honest coset proof rejected"
    );
}

#[test]
fn honest_proofs_verify_across_sizes() {
    for (log_n, log_blowup, degree) in [(4u32, 1u32, 4usize), (6, 2, 8), (8, 3, 16)] {
        let codeword = low_degree_codeword(log_n, degree, Fp::ONE, 0x1000 + log_n as u64);
        let proof = fri_prove(&codeword, Fp::ONE, log_blowup, 32);
        assert!(
            fri_verify(&proof, Fp::ONE, log_n, log_blowup, 32),
            "honest proof at log_n {log_n} rejected"
        );
    }
}

#[test]
fn a_high_degree_codeword_is_rejected() {
    // A random codeword is far from any low-degree polynomial; its folded final
    // layer is not constant, so the low-degree check rejects it.
    let (log_n, log_blowup, queries) = (5u32, 2u32, 40usize);
    let codeword = random_codeword(log_n, 0x99);
    let proof = fri_prove(&codeword, Fp::ONE, log_blowup, queries);
    assert!(!fri_verify(&proof, Fp::ONE, log_n, log_blowup, queries), "a random codeword verified");
}

#[test]
fn a_forged_constant_final_layer_is_rejected() {
    // Forge the final layer to a constant so the low-degree check passes. This
    // changes the Fiat-Shamir transcript, so the pre-committed query openings no
    // longer land at the re-derived positions and the Merkle checks fail.
    let (log_n, log_blowup, queries) = (5u32, 2u32, 40usize);
    let codeword = random_codeword(log_n, 0x1357);
    let mut proof = fri_prove(&codeword, Fp::ONE, log_blowup, queries);
    let constant = proof.final_layer[0];
    for value in proof.final_layer.iter_mut() {
        *value = constant;
    }
    assert!(
        !fri_verify(&proof, Fp::ONE, log_n, log_blowup, queries),
        "a forged constant final verified"
    );
}

#[test]
fn a_tampered_opening_is_rejected() {
    // Corrupt one opened value. Its Merkle path no longer recomputes the root.
    let (log_n, log_blowup, queries) = (5u32, 2u32, 40usize);
    let codeword = low_degree_codeword(log_n, 8, Fp::ONE, 0x2468);
    let mut proof = fri_prove(&codeword, Fp::ONE, log_blowup, queries);
    proof.queries[0].layers[0].a = proof.queries[0].layers[0].a + Fp::ONE;
    assert!(
        !fri_verify(&proof, Fp::ONE, log_n, log_blowup, queries),
        "a tampered opening verified"
    );
}

// The extension-field fold, drawn from Fp2, is what makes FRI money-grade: the
// soundness error drops from ~2^-64 (base field) to ~2^-128. These check the new
// fold against the already-verified base fold and the degree-halving property.

#[test]
fn the_extension_fold_faithfully_extends_the_base_fold() {
    // On a base-field challenge, fold_first must reproduce fold_layer embedded
    // into Fp2 exactly. This ties the new path to the verified one.
    let log_n = 4u32;
    let n = 1usize << log_n;
    let omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();
    let shift = Fp::from_u64(7);
    let mut s = 0xC0FFEE123u64 | 1;
    let evals: Vec<Fp> = (0..n).map(|_| Fp::from_u64(xorshift(&mut s))).collect();
    let beta = Fp::from_u64(xorshift(&mut s));

    let base = fold_layer(&evals, beta, shift, omega, inv2);
    let ext = fold_first(&evals, Fp2::from_base(beta), shift, omega, inv2);
    assert_eq!(base.len(), ext.len());
    for (b, e) in base.iter().zip(ext.iter()) {
        assert_eq!(Fp2::from_base(*b), *e);
    }
}

#[test]
fn a_constant_codeword_folds_to_a_constant_in_the_extension() {
    let log_n = 4u32;
    let n = 1usize << log_n;
    let omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();
    let shift = Fp::from_u64(3);
    let c = Fp2::new(Fp::from_u64(0xDEAD), Fp::from_u64(0xBEEF));
    let cw: Vec<Fp2> = vec![c; n];
    let folded = fold_ext(&cw, Fp2::new(Fp::from_u64(9), Fp::from_u64(11)), shift, omega, inv2);
    assert_eq!(folded.len(), n / 2);
    for v in &folded {
        assert_eq!(*v, c);
    }
}

#[test]
fn the_extension_fold_chain_halves_each_layer() {
    let log_n = 5u32;
    let n = 1usize << log_n;
    let omega = root_of_unity(log_n);
    let inv2 = Fp::from_u64(2).inv();
    let shift = Fp::from_u64(5);
    let mut s = 0xABCDEF01u64 | 1;
    let evals: Vec<Fp> = (0..n).map(|_| Fp::from_u64(xorshift(&mut s))).collect();

    let b0 = Fp2::new(Fp::from_u64(0x33), Fp::from_u64(0x44));
    let l1 = fold_first(&evals, b0, shift, omega, inv2);
    let b1 = Fp2::new(Fp::from_u64(0x55), Fp::from_u64(0x66));
    let l2 = fold_ext(&l1, b1, shift.square(), omega.square(), inv2);
    assert_eq!(l1.len(), n / 2);
    assert_eq!(l2.len(), n / 4);
}
