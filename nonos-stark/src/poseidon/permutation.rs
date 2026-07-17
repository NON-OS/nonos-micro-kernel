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

//! The Poseidon-Goldilocks permutation. Each round adds the round constants,
//! applies the x^7 S-box (to every lane in a full round, to lane 0 only in a
//! partial round), and mixes through the MDS matrix. The schedule is the
//! standard four full, twenty-two partial, four full. This is the reference
//! (unoptimized) permutation, equal by construction to the optimized Plonky2
//! one and checked against the published test vectors.

use super::super::field::Fp;
use super::constants::{
    FULL_ROUNDS, MDS_CIRC, MDS_DIAG, PARTIAL_ROUNDS, ROUND_CONSTANTS, WIDTH,
};

/// The x^7 S-box on a single lane.
#[inline]
pub(super) fn sbox(x: Fp) -> Fp {
    let x2 = x.square();
    let x4 = x2.square();
    let x3 = x * x2;
    x3 * x4
}

/// Add the round constants for `round` to the state in place.
#[inline]
fn add_round_constants(state: &mut [Fp; WIDTH], round: usize) {
    for (i, lane) in state.iter_mut().enumerate() {
        *lane = *lane + Fp::from_u64(ROUND_CONSTANTS[round * WIDTH + i]);
    }
}

/// The MDS mix: row `r` of the output is the circulant combination of the
/// state plus the diagonal term, `sum_i state[(i + r) % W] * CIRC[i] +
/// state[r] * DIAG[r]`. The circulant is invertible over the field, so the
/// layer is a bijection.
#[inline]
fn mds(state: &[Fp; WIDTH]) -> [Fp; WIDTH] {
    let mut out = [Fp::ZERO; WIDTH];
    for (r, o) in out.iter_mut().enumerate() {
        let mut acc = Fp::ZERO;
        for (i, &c) in MDS_CIRC.iter().enumerate() {
            acc = acc + state[(i + r) % WIDTH] * Fp::from_u64(c);
        }
        acc = acc + state[r] * Fp::from_u64(MDS_DIAG[r]);
        *o = acc;
    }
    out
}

/// One full round: constants, S-box on every lane, MDS.
#[inline]
fn full_round(state: &mut [Fp; WIDTH], round: usize) {
    add_round_constants(state, round);
    for lane in state.iter_mut() {
        *lane = sbox(*lane);
    }
    *state = mds(state);
}

/// One partial round: constants, S-box on lane 0 only, MDS.
#[inline]
fn partial_round(state: &mut [Fp; WIDTH], round: usize) {
    add_round_constants(state, round);
    state[0] = sbox(state[0]);
    *state = mds(state);
}

/// Apply the full Poseidon permutation to a width-12 state.
pub fn permute(mut state: [Fp; WIDTH]) -> [Fp; WIDTH] {
    let mut round = 0;
    for _ in 0..(FULL_ROUNDS / 2) {
        full_round(&mut state, round);
        round += 1;
    }
    for _ in 0..PARTIAL_ROUNDS {
        partial_round(&mut state, round);
        round += 1;
    }
    for _ in 0..(FULL_ROUNDS / 2) {
        full_round(&mut state, round);
        round += 1;
    }
    state
}
