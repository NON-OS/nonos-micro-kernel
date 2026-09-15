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

#[inline]
pub(crate) fn ct_lt_u64(a: u64, b: u64) -> u64 {
    let x = a ^ ((a ^ b) | (a.wrapping_sub(b) ^ b));
    x >> 63
}

impl FieldElement {
    pub fn is_even(&self) -> bool {
        self.0[0] & 1 == 0
    }

    pub fn negate(&self) -> Self {
        let mut result = [0u64; 4];
        let mut borrow = 0i128;

        for (out, (a, b)) in result.iter_mut().zip(Self::P.iter().zip(self.0.iter())) {

            borrow += *a as i128 - *b as i128;

            if borrow < 0 {

                *out = (borrow + (1i128 << 64)) as u64;

                borrow = -1;

            } else {

                *out = borrow as u64;

                borrow = 0;

            }

        }

        let is_zero_mask = 0u64.wrapping_sub(self.ct_is_zero());
        let neg = Self(result);
        Self::ct_select(is_zero_mask, &Self::ZERO, &neg)
    }

    pub fn ct_select(mask: u64, a: &Self, b: &Self) -> Self {
        Self([
            (a.0[0] & mask) | (b.0[0] & !mask),
            (a.0[1] & mask) | (b.0[1] & !mask),
            (a.0[2] & mask) | (b.0[2] & !mask),
            (a.0[3] & mask) | (b.0[3] & !mask),
        ])
    }

    pub fn ct_is_zero(&self) -> u64 {
        let or = self.0[0] | self.0[1] | self.0[2] | self.0[3];
        let is_zero = (or | or.wrapping_neg()) >> 63;
        1 ^ is_zero
    }

    pub fn ct_eq(&self, other: &Self) -> u64 {
        let diff = self.sub(other);
        diff.ct_is_zero()
    }
}
