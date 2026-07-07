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
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::fri_poseidon::{fri_prove, fri_verify};
use crate::crypto::stark::poly::eval;

extern crate alloc;
use alloc::vec::Vec;

// The Poseidon-committed FRI: the same low-degree test, but the commitments and
// the transcript are algebraic. A proof made here is verifiable inside a STARK,
// which the BLAKE3 version is not. Soundness is checked the same way: honest
// low-degree codewords pass, and a high-degree codeword and a tampered opening
// fail.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn hasher() -> Poseidon {
    Poseidon::new(3, [Fp::ZERO; RATE])
}

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

#[test]
fn an_honest_low_degree_codeword_verifies() {
    let (log_n, log_blowup, degree, queries) = (5u32, 2u32, 8usize, 30usize);
    let shift = Fp::from_u64(7);
    let h = hasher();
    let codeword = low_degree_codeword(log_n, degree, shift, 0xABCD);
    let proof = fri_prove(&codeword, shift, log_blowup, queries, &h);
    assert!(fri_verify(&proof, shift, log_n, log_blowup, queries, &h), "honest proof rejected");
}

#[test]
fn a_high_degree_codeword_is_rejected() {
    let (log_n, log_blowup, queries) = (5u32, 2u32, 30usize);
    let shift = Fp::from_u64(7);
    let h = hasher();
    let n = 1usize << log_n;
    let mut s = 0x99u64 | 1;
    let codeword: Vec<Fp> = (0..n).map(|_| Fp::from_u64(xorshift(&mut s))).collect();
    let proof = fri_prove(&codeword, shift, log_blowup, queries, &h);
    assert!(
        !fri_verify(&proof, shift, log_n, log_blowup, queries, &h),
        "a random codeword verified"
    );
}

#[test]
fn a_tampered_opening_is_rejected() {
    let (log_n, log_blowup, queries) = (5u32, 2u32, 30usize);
    let shift = Fp::from_u64(7);
    let h = hasher();
    let codeword = low_degree_codeword(log_n, 8, shift, 0x2468);
    let mut proof = fri_prove(&codeword, shift, log_blowup, queries, &h);
    proof.queries[0].layers[0].a = proof.queries[0].layers[0].a + Fp::ONE;
    assert!(
        !fri_verify(&proof, shift, log_n, log_blowup, queries, &h),
        "a tampered opening verified"
    );
}
