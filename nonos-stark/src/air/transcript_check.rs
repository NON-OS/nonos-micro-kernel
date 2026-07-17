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

//! The transcript-derivation gadget: proving a proof's Fiat-Shamir challenges were
//! honestly squeezed from its committed data, the last thing a recursive verifier
//! needs so the challenges are proven, not trusted. It runs the exact Poseidon
//! sponge of `PoseidonTranscript`: an absorb adds its value into the first lane and
//! permutes, a squeeze reads the first lane and permutes. Each operation is one
//! permutation, arithmetized round by round; the absorbed values ride a periodic
//! column, the squeezed challenges are pinned by boundaries, and the sponge starts
//! empty. A verifier feeds the absorbed sequence (commitment roots, out-of-domain
//! frame) and the claimed challenges (coefficients, out-of-domain point,
//! DEEP coefficients), and the proof binds them to the one honest transcript.

use super::super::field::{Felt, Fp};
use super::poseidon::{Poseidon, WIDTH};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

/// One transcript operation: absorb a value, or squeeze the challenge it carries.
pub enum TranscriptOp {
    Absorb(Fp),
    Squeeze(Fp),
}

pub struct TranscriptCheck {
    hasher: Poseidon,
    log_rounds: u32,
    ops: Vec<TranscriptOp>,
    witness_inject: bool,
}

impl TranscriptCheck {
    /// Build the check over `ops` under the sponge `hasher`, whose permutation is
    /// `2^log_rounds` rounds. The absorbed values ride a periodic column and the
    /// squeezed challenges are pinned as boundaries: instance-specific structure,
    /// fine for a per-proof AIR.
    pub fn new(hasher: Poseidon, log_rounds: u32, ops: Vec<TranscriptOp>) -> TranscriptCheck {
        TranscriptCheck { hasher, log_rounds, ops, witness_inject: false }
    }

    /// The production form: the absorbed value rides the trace (one extra column),
    /// gated by a structural inject selector, and the squeezes are not pinned. The
    /// AIR is then instance-independent (round constants and the selector are the
    /// only periodic columns, the sponge-empty rows the only boundaries); the
    /// absorbed values and squeezed challenges are witness, bound by the assembly's
    /// grand product to their sources and consumers.
    pub fn new_witness(
        hasher: Poseidon,
        log_rounds: u32,
        ops: Vec<TranscriptOp>,
    ) -> TranscriptCheck {
        TranscriptCheck { hasher, log_rounds, ops, witness_inject: true }
    }

    fn rounds(&self) -> usize {
        1usize << self.log_rounds
    }

    fn inject_at(&self, row: usize) -> Fp {
        let l = self.rounds();
        if row.is_multiple_of(l) && row / l < self.ops.len() {
            if let TranscriptOp::Absorb(v) = self.ops[row / l] {
                return v;
            }
        }
        Fp::ZERO
    }

    /// The structural inject selector: one at every absorb-operation start, zero
    /// elsewhere. Instance-independent: it depends only on the fixed op schedule,
    /// not on the absorbed values.
    fn inject_sel(&self, row: usize) -> Fp {
        let l = self.rounds();
        if row.is_multiple_of(l) && row / l < self.ops.len() {
            if let TranscriptOp::Absorb(_) = self.ops[row / l] {
                return Fp::ONE;
            }
        }
        Fp::ZERO
    }

    /// The witness: the sponge state at every round, the absorbed values injected
    /// into the first lane at each operation start. Padding rows keep permuting. In
    /// the witness form the absorbed value is also written to the extra trace column.
    pub fn trace(&self) -> Vec<Fp> {
        let n = 1usize << self.log_trace_len();
        let l = self.rounds();
        let w = self.trace_width();
        let mut state = [Fp::ZERO; WIDTH];
        let mut tr = alloc::vec![Fp::ZERO; n * w];
        for row in 0..n {
            tr[row * w..row * w + WIDTH].copy_from_slice(&state);
            let injv = self.inject_at(row);
            if self.witness_inject {
                tr[row * w + WIDTH] = injv;
            }
            let mut inj = state;
            inj[0] = inj[0] + injv;
            state = self.hasher.round_with_rc(&inj, &self.hasher.round_constant(row % l));
        }
        tr
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let w = self.trace_width();
        let mut state = [F::ZERO; WIDTH];
        state.copy_from_slice(&window[..WIDTH]);
        let mut rc = [F::ZERO; WIDTH];
        rc.copy_from_slice(&periodic[..WIDTH]);
        // The absorbed value enters additively so the sponge stays degree 7. In the
        // per-proof form it is the periodic column; in the production form it is the
        // witnessed trace column, added directly (not multiplied by the selector,
        // which would raise the degree through the S-box).
        if self.witness_inject {
            state[0] = state[0] + window[WIDTH];
        } else {
            state[0] = state[0] + periodic[WIDTH];
        }
        let pr = self.hasher.round_generic(&state, &rc);
        let mut out = Vec::with_capacity(WIDTH + 1);
        for (j, next) in window[w..w + WIDTH].iter().enumerate() {
            out.push(*next - pr[j]);
        }
        // The witnessed value must be zero except at an absorb start, so it cannot
        // inject off schedule. The selector is one there, zero elsewhere, so this
        // masks the value on every non-absorb row. Degree two, well under the S-box.
        if self.witness_inject {
            out.push((F::ONE - periodic[WIDTH]) * window[WIDTH]);
        }
        out
    }
}

impl AirExt for TranscriptCheck {
    fn transition_ext(
        &self,
        window: &[super::super::field::Fp2],
        periodic: &[super::super::field::Fp2],
    ) -> Vec<super::super::field::Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for TranscriptCheck {
    fn log_trace_len(&self) -> u32 {
        (self.ops.len() * self.rounds()).next_power_of_two().trailing_zeros()
    }

    fn trace_width(&self) -> usize {
        if self.witness_inject {
            WIDTH + 1
        } else {
            WIDTH
        }
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        7
    }

    fn num_transition(&self) -> usize {
        if self.witness_inject {
            WIDTH + 1
        } else {
            WIDTH
        }
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let n = 1usize << self.log_trace_len();
        let l = self.rounds();
        let mut cols: Vec<Vec<Fp>> = (0..WIDTH + 1).map(|_| alloc::vec![Fp::ZERO; n]).collect();
        for row in 0..n {
            let rc = self.hasher.round_constant(row % l);
            for (j, col) in cols.iter_mut().take(WIDTH).enumerate() {
                col[row] = rc[j];
            }
            // The last periodic column is the absorbed value (per-proof form) or the
            // structural inject selector (production form).
            cols[WIDTH][row] =
                if self.witness_inject { self.inject_sel(row) } else { self.inject_at(row) };
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let mut b = Vec::new();
        // The sponge starts empty.
        for j in 0..WIDTH {
            b.push((j, 0, Fp::ZERO));
        }
        // In the per-proof form each squeeze pins its challenge as a boundary. In the
        // production form the challenge is a witness cell (lane 0 at the squeeze row),
        // bound by the assembly's grand product to its consumer, so nothing is pinned.
        if !self.witness_inject {
            let l = self.rounds();
            for (oi, op) in self.ops.iter().enumerate() {
                if let TranscriptOp::Squeeze(c) = op {
                    b.push((0, oi * l, *c));
                }
            }
        }
        b
    }
}
