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

//! A STARK proof of knowledge of a Poseidon-Goldilocks preimage. The trace is
//! the permutation state, one row per round: row 0 is the input, row r+1 is the
//! state after round r, so the thirty rounds occupy rows 0 through 30. Each
//! transition enforces one real Poseidon round, `next = MDS(sbox(state + rc))`,
//! with the round constants and the full-versus-partial S-box supplied as
//! public periodic columns, so the constraint is the published permutation and
//! not a stand-in. The boundary pins the input capacity lanes to zero, the
//! sponge initialization, and the output rate lanes to a public digest. A
//! satisfying trace is a preimage the proof never reveals.

use super::super::field::Fp;
use super::super::poseidon::constants::{
    FULL_ROUNDS, MDS_CIRC, MDS_DIAG, N_ROUNDS, PARTIAL_ROUNDS, ROUND_CONSTANTS, WIDTH,
};
use super::spec::Air;
use alloc::vec;
use alloc::vec::Vec;

/// Log2 of the padded trace length. Thirty rounds need thirty-one state rows,
/// so the trace is padded to thirty-two.
pub(crate) const LOG_TRACE: u32 = 5;
const TRACE_LEN: usize = 1 << LOG_TRACE;
/// The sponge rate; the first `RATE` lanes carry the absorbed input and, at the
/// output, the digest.
pub(crate) const RATE: usize = 8;
/// The digest width pinned as public output.
pub(crate) const DIGEST: usize = 4;

/// Which rounds apply the S-box to every lane. The schedule is
/// `FULL_ROUNDS / 2` full, then all partial, then `FULL_ROUNDS / 2` full, so a
/// round is full exactly when it is outside the partial-round span.
fn is_full_round(round: usize) -> bool {
    let partial = FULL_ROUNDS / 2..FULL_ROUNDS / 2 + PARTIAL_ROUNDS;
    !partial.contains(&round)
}

/// Prove knowledge of a Poseidon preimage whose output rate lanes are `digest`.
pub struct PoseidonPreimage {
    pub digest: [Fp; DIGEST],
}

impl PoseidonPreimage {
    fn sbox(x: Fp) -> Fp {
        let x2 = x.square();
        let x4 = x2.square();
        x * x2 * x4
    }
}

/// One real Poseidon round applied to a state row, used to build the trace so
/// the trace and the AIR transition are the same round function by
/// construction.
fn round(state: &[Fp; WIDTH], r: usize) -> [Fp; WIDTH] {
    let full = is_full_round(r);
    let mut post = [Fp::ZERO; WIDTH];
    for i in 0..WIDTH {
        let added = state[i] + Fp::from_u64(ROUND_CONSTANTS[r * WIDTH + i]);
        post[i] = if i == 0 || full { PoseidonPreimage::sbox(added) } else { added };
    }
    let mut out = [Fp::ZERO; WIDTH];
    for (r2, o) in out.iter_mut().enumerate() {
        let mut acc = Fp::ZERO;
        for (i, &c) in MDS_CIRC.iter().enumerate() {
            acc = acc + post[(i + r2) % WIDTH] * Fp::from_u64(c);
        }
        acc = acc + post[r2] * Fp::from_u64(MDS_DIAG[r2]);
        *o = acc;
    }
    out
}

/// Build the execution trace and the resulting digest for a rate input. The
/// capacity lanes are initialized to zero, each round advances one row, and the
/// padding rows repeat the output. Row-major, `WIDTH` columns.
pub fn poseidon_preimage_trace(rate_input: [Fp; RATE]) -> (Vec<Fp>, [Fp; DIGEST]) {
    let mut state = [Fp::ZERO; WIDTH];
    state[..RATE].copy_from_slice(&rate_input);

    let mut rows: Vec<[Fp; WIDTH]> = Vec::with_capacity(TRACE_LEN);
    rows.push(state);
    for r in 0..N_ROUNDS {
        state = round(&state, r);
        rows.push(state);
    }
    while rows.len() < TRACE_LEN {
        rows.push(state);
    }

    let mut flat = Vec::with_capacity(TRACE_LEN * WIDTH);
    for row in &rows {
        flat.extend_from_slice(row);
    }
    let mut digest = [Fp::ZERO; DIGEST];
    digest.copy_from_slice(&rows[N_ROUNDS][..DIGEST]);
    (flat, digest)
}

impl Air for PoseidonPreimage {
    fn log_trace_len(&self) -> u32 {
        LOG_TRACE
    }

    fn trace_width(&self) -> usize {
        WIDTH
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // The S-box is degree seven; the selector and constants are periodic
        // scalars, so they do not raise the degree in the trace variables.
        7
    }

    fn num_transition(&self) -> usize {
        WIDTH
    }

    /// The periodic columns, one per row of the padded trace: the twelve round
    /// constants, a full-round selector, and an active selector that is zero on
    /// the padding rows so their transition is the identity.
    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let mut cols: Vec<Vec<Fp>> = vec![vec![Fp::ZERO; TRACE_LEN]; WIDTH + 2];
        for round in 0..N_ROUNDS {
            for lane in 0..WIDTH {
                cols[lane][round] = Fp::from_u64(ROUND_CONSTANTS[round * WIDTH + lane]);
            }
            cols[WIDTH][round] = if is_full_round(round) { Fp::ONE } else { Fp::ZERO };
            cols[WIDTH + 1][round] = Fp::ONE;
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let cur = &window[0..WIDTH];
        let next = &window[WIDTH..2 * WIDTH];
        let rc = &periodic[0..WIDTH];
        let is_full = periodic[WIDTH];
        let is_active = periodic[WIDTH + 1];

        // Post S-box lane: on a full round every lane, on a partial round lane
        // zero only. `is_full` selects between the two arms without a branch,
        // matching the real permutation.
        let mut post = [Fp::ZERO; WIDTH];
        for i in 0..WIDTH {
            let added = cur[i] + rc[i];
            post[i] = if i == 0 {
                Self::sbox(added)
            } else {
                is_full * Self::sbox(added) + (Fp::ONE - is_full) * added
            };
        }

        // The MDS mix, `out[r] = sum_i post[(i + r) % W] * CIRC[i] + post[r] *
        // DIAG[r]`, the same circulant plus diagonal the permutation uses.
        let mut mixed = [Fp::ZERO; WIDTH];
        for (r, m) in mixed.iter_mut().enumerate() {
            let mut acc = Fp::ZERO;
            for (i, &c) in MDS_CIRC.iter().enumerate() {
                acc = acc + post[(i + r) % WIDTH] * Fp::from_u64(c);
            }
            acc = acc + post[r] * Fp::from_u64(MDS_DIAG[r]);
            *m = acc;
        }

        // On active rows the next state is the round output; on padding rows it
        // is the identity, so the padded tail carries no constraint of its own.
        let mut out = Vec::with_capacity(WIDTH);
        for i in 0..WIDTH {
            let target = is_active * mixed[i] + (Fp::ONE - is_active) * cur[i];
            out.push(next[i] - target);
        }
        out
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let mut b = Vec::with_capacity((WIDTH - RATE) + DIGEST);
        // The input capacity lanes start at zero: the sponge initialization.
        for lane in RATE..WIDTH {
            b.push((lane, 0, Fp::ZERO));
        }
        // The output rate lanes at row N_ROUNDS are the public digest.
        for (i, d) in self.digest.iter().enumerate() {
            b.push((i, N_ROUNDS, *d));
        }
        b
    }
}
