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
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::fri_ext::{fri_prove_ext, fri_verify_ext};
use crate::crypto::stark::poly::eval;

extern crate alloc;
use alloc::vec::Vec;

/// Lift a base codeword into the extension, as a STARK does before the money-grade
/// FRI (the DEEP quotient is already `Fp2` and passed directly).
fn lift(cw: &[Fp]) -> Vec<Fp2> {
    cw.iter().map(|v| Fp2::from_base(*v)).collect()
}

// The money-grade FRI: extension-field fold challenges (~2^-128 soundness) plus a
// grinding proof-of-work. These check completeness (honest low-degree codewords
// pass), soundness (high-degree and tampered proofs fail), and that the grinding
// and Fiat-Shamir binding actually bite, all against the real prover and verifier.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn low_degree_codeword(log_n: u32, d: usize, shift: Fp, seed: u64) -> Vec<Fp> {
    let n = 1usize << log_n;
    let omega = root_of_unity(log_n);
    let mut s = seed | 1;
    let coeffs: Vec<Fp> = (0..d).map(|_| Fp::from_u64(xorshift(&mut s))).collect();
    let mut x = shift;
    let mut cw = Vec::with_capacity(n);
    for _ in 0..n {
        cw.push(eval(&coeffs, x));
        x = x * omega;
    }
    cw
}

fn random_codeword(log_n: u32, seed: u64) -> Vec<Fp> {
    let n = 1usize << log_n;
    let mut s = seed | 1;
    (0..n).map(|_| Fp::from_u64(xorshift(&mut s))).collect()
}

#[test]
fn an_honest_low_degree_codeword_verifies() {
    let (log_n, log_blowup, degree, queries, grind) = (5u32, 2u32, 8usize, 32usize, 8u32);
    let cw = low_degree_codeword(log_n, degree, Fp::ONE, 0xABCD);
    let proof = fri_prove_ext(&lift(&cw), Fp::ONE, log_blowup, queries, grind);
    assert!(
        fri_verify_ext(&proof, Fp::ONE, log_n, log_blowup, queries, grind),
        "an honest extension proof was rejected"
    );
}

#[test]
fn honest_proofs_verify_on_a_coset_across_sizes() {
    for (log_n, log_blowup, degree) in [(4u32, 1u32, 4usize), (6, 2, 8), (8, 3, 16)] {
        let shift = Fp::from_u64(7);
        let cw = low_degree_codeword(log_n, degree, shift, 0x2000 + log_n as u64);
        let proof = fri_prove_ext(&lift(&cw), shift, log_blowup, 32, 8);
        assert!(
            fri_verify_ext(&proof, shift, log_n, log_blowup, 32, 8),
            "an honest coset proof at log_n {log_n} was rejected"
        );
    }
}

#[test]
fn a_high_degree_codeword_is_rejected() {
    let (log_n, log_blowup, queries, grind) = (5u32, 2u32, 32usize, 8u32);
    let cw = random_codeword(log_n, 0x99);
    let proof = fri_prove_ext(&lift(&cw), Fp::ONE, log_blowup, queries, grind);
    assert!(
        !fri_verify_ext(&proof, Fp::ONE, log_n, log_blowup, queries, grind),
        "a random codeword verified"
    );
}

#[test]
fn a_tampered_opening_is_rejected() {
    let (log_n, log_blowup, queries, grind) = (5u32, 2u32, 32usize, 8u32);
    let cw = low_degree_codeword(log_n, 8, Fp::ONE, 0x2468);
    let mut proof = fri_prove_ext(&lift(&cw), Fp::ONE, log_blowup, queries, grind);
    proof.queries[0].layers[0].a =
        proof.queries[0].layers[0].a + crate::crypto::stark::field::Fp2::ONE;
    assert!(
        !fri_verify_ext(&proof, Fp::ONE, log_n, log_blowup, queries, grind),
        "a tampered extension opening verified"
    );
}

#[test]
fn a_forged_grinding_nonce_is_rejected() {
    // Grinding is enforced: corrupt the proof-of-work nonce and the verifier must
    // reject even an otherwise-honest proof.
    let (log_n, log_blowup, queries, grind) = (5u32, 2u32, 32usize, 12u32);
    let cw = low_degree_codeword(log_n, 8, Fp::ONE, 0x1111);
    let mut proof = fri_prove_ext(&lift(&cw), Fp::ONE, log_blowup, queries, grind);
    assert!(fri_verify_ext(&proof, Fp::ONE, log_n, log_blowup, queries, grind), "honest rejected");
    proof.pow_nonce = proof.pow_nonce.wrapping_add(1);
    assert!(
        !fri_verify_ext(&proof, Fp::ONE, log_n, log_blowup, queries, grind),
        "a forged grinding nonce verified"
    );
}
