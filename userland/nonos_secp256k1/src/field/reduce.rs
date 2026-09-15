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

use super::types::FieldElement;

impl FieldElement {
    pub(crate) fn reduce(&mut self) {
        let mut borrow = 0i128;
        let mut temp = [0u64; 4];

        for (out, (a, b)) in temp.iter_mut().zip(self.0.iter().zip(Self::P.iter())) {

            borrow += *a as i128 - *b as i128;

            if borrow < 0 {

                *out = (borrow + (1i128 << 64)) as u64;

                borrow = -1;

            } else {

                *out = borrow as u64;

                borrow = 0;

            }

        }

        let no_borrow = ((borrow >> 127) & 1) as u64;
        let mask = no_borrow.wrapping_sub(1);
        for (out, t) in self.0.iter_mut().zip(temp.iter()) {
            *out = (*t & mask) | (*out & !mask);
        }
    }

    pub(crate) fn reduce_wide(&self, wide: &[u128; 8]) -> Self {
        let c: u64 = 0x1000003D1;
        let mut acc = [0u128; 5];

        acc[..4].copy_from_slice(&wide[..4]);

        for i in 4..8 {
            let hi = wide[i];
            acc[i - 4] += (hi & 0xFFFFFFFFFFFFFFFF) * c as u128;
            if i < 7 {
                acc[i - 3] += hi >> 64;
            }
        }

        for i in 0..4 {
            acc[i + 1] += acc[i] >> 64;
            acc[i] &= 0xFFFFFFFFFFFFFFFF;
        }

        // Fold carry from acc[4]: 2^256 ≡ c (mod p)
        let carry = acc[4] * c as u128;
        acc[0] = (acc[0] & 0xFFFFFFFFFFFFFFFF) + carry;
        acc[1] = (acc[1] & 0xFFFFFFFFFFFFFFFF) + (acc[0] >> 64);
        acc[0] &= 0xFFFFFFFFFFFFFFFF;
        acc[2] = (acc[2] & 0xFFFFFFFFFFFFFFFF) + (acc[1] >> 64);
        acc[1] &= 0xFFFFFFFFFFFFFFFF;
        acc[3] = (acc[3] & 0xFFFFFFFFFFFFFFFF) + (acc[2] >> 64);
        acc[2] &= 0xFFFFFFFFFFFFFFFF;

        let mut result = Self([acc[0] as u64, acc[1] as u64, acc[2] as u64, acc[3] as u64]);
        result.reduce();
        result
    }
}
