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

//! Arithmetizing Fiat-Shamir: an AIR that proves a challenge was squeezed from a
//! sequence of absorbed values through a Poseidon sponge. Each block absorbs one
//! value into the first lane and permutes; the AIR runs the same state, with a
//! Poseidon round within a block and an absorb-then-carry at each boundary. The
//! first absorbed value seeds the state and the final squeezed lane is pinned to
//! the public challenge. A recursive verifier needs this to derive its own
//! challenges in circuit; a bitwise transcript hash could not be proven, an
//! algebraic one can.

use super::super::field::{Felt, Fp, Fp2};
use super::poseidon::{Poseidon, WIDTH};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

pub struct FiatShamir {
    hasher: Poseidon,
    log_rounds: u32,
    log_slots: u32,
    /// The absorbed values, one per block.
    inputs: Vec<Fp>,
    /// The squeezed challenge (first lane after the final permutation).
    challenge: Fp,
}

impl FiatShamir {
    pub fn new(
        hasher: Poseidon,
        log_rounds: u32,
        log_slots: u32,
        inputs: Vec<Fp>,
        challenge: Fp,
    ) -> FiatShamir {
        FiatShamir { hasher, log_rounds, log_slots, inputs, challenge }
    }

    fn rounds(&self) -> usize {
        1usize << self.log_rounds
    }

    fn blocks(&self) -> usize {
        (1usize << self.log_slots) - 1
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let mut state = [F::ZERO; WIDTH];
        state.copy_from_slice(&window[..WIDTH]);
        let mut rc = [F::ZERO; WIDTH];
        rc.copy_from_slice(&periodic[..WIDTH]);
        let bnd = periodic[WIDTH];
        let input = periodic[WIDTH + 1];

        let pr = self.hasher.round_generic(&state, &rc);

        let mut out = Vec::with_capacity(WIDTH);
        for (j, next) in window[WIDTH..2 * WIDTH].iter().enumerate() {
            let extra = if j == 0 { bnd * input } else { F::ZERO };
            out.push(*next - (pr[j] + extra));
        }
        out
    }
}

impl AirExt for FiatShamir {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for FiatShamir {
    fn log_trace_len(&self) -> u32 {
        self.log_rounds + self.log_slots
    }

    fn trace_width(&self) -> usize {
        WIDTH
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        7
    }

    fn num_transition(&self) -> usize {
        WIDTH
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let l = self.rounds();
        let blocks = self.blocks();
        let n = 1usize << self.log_trace_len();
        // WIDTH round-constant columns, a boundary selector, and the absorbed value.
        let mut cols: Vec<Vec<Fp>> = (0..WIDTH + 2).map(|_| Vec::with_capacity(n)).collect();
        for r in 0..n {
            let rc = self.hasher.round_constant(r % l);
            for (j, col) in cols.iter_mut().take(WIDTH).enumerate() {
                col.push(rc[j]);
            }
            let is_boundary = r % l == l - 1 && r < blocks * l;
            cols[WIDTH].push(if is_boundary { Fp::ONE } else { Fp::ZERO });
            // The block being formed absorbs its input; the final block absorbs
            // nothing and just carries the squeeze forward.
            let m = (r + 1) / l;
            let input = if is_boundary && m < blocks { self.inputs[m] } else { Fp::ZERO };
            cols[WIDTH + 1].push(input);
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let l = self.rounds();
        let blocks = self.blocks();
        let mut b = Vec::with_capacity(WIDTH + 1);
        // The state starts empty with the first value absorbed into lane zero.
        b.push((0, 0, self.inputs[0]));
        for j in 1..WIDTH {
            b.push((j, 0, Fp::ZERO));
        }
        // The final squeeze is the public challenge.
        b.push((0, blocks * l, self.challenge));
        b
    }
}
