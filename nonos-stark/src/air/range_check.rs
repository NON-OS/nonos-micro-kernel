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

//! A range-check AIR by bit decomposition: two columns, a running remainder and a
//! per-row bit, with `acc[i] = 2*acc[i+1] + bit[i]` and each bit boolean, pinned to
//! zero at the last row. It proves `acc[0]` is the sum of `2^t - 1` bits, i.e. it
//! lies in `[0, 2^(2^log_t - 1))`, the overflow/underflow guard a value must pass.
//! Written once over `Felt`, so it is proven at money-grade soundness. The bit
//! constraint is `Nonos.Stark.Instance`'s gate, `w*(w-1)`.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec;
use alloc::vec::Vec;

pub struct RangeCheck {
    pub log_t: u32,
}

impl RangeCheck {
    /// The two constraints over any field: the remainder peels one bit per row,
    /// and each bit is boolean. `window = [acc_i, bit_i, acc_next, bit_next]`.
    fn transition_impl<F: Felt>(&self, window: &[F], _periodic: &[F]) -> Vec<F> {
        let acc_i = window[0];
        let bit_i = window[1];
        let acc_next = window[2];
        let two = F::from_base(Fp::from_u64(2));
        let one = F::ONE;
        vec![acc_i - two * acc_next - bit_i, bit_i * (bit_i - one)]
    }
}

impl Air for RangeCheck {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }

    fn trace_width(&self) -> usize {
        2
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        2
    }

    fn num_transition(&self) -> usize {
        2
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // The remainder is fully peeled: the value fits in the decomposed bits.
        let last = (1usize << self.log_t) - 1;
        vec![(0, last, Fp::ZERO)]
    }
}

impl AirExt for RangeCheck {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}
