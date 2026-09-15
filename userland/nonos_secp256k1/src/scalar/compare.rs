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

use super::types::Scalar;

#[inline]
fn ct_lt_u64(a: u64, b: u64) -> u64 {
    let x = a ^ ((a ^ b) | (a.wrapping_sub(b) ^ b));
    x >> 63
}

pub(crate) fn is_valid(s: &Scalar) -> bool {
    let mut lt: u64 = 0;
    let mut eq: u64 = 1;

    for i in (0..4).rev() {
        let a = s.0[i];
        let n = Scalar::N[i];
        let a_lt_n = ct_lt_u64(a, n);
        let a_gt_n = ct_lt_u64(n, a);
        lt |= eq & a_lt_n;
        eq &= (1 - a_lt_n) & (1 - a_gt_n);
    }

    lt == 1
}

impl Scalar {
    pub fn is_zero(&self) -> bool {
        self.0 == [0, 0, 0, 0]
    }

    pub fn ct_is_zero(&self) -> u64 {
        let or = self.0[0] | self.0[1] | self.0[2] | self.0[3];
        let is_nonzero = (or | or.wrapping_neg()) >> 63;
        1 ^ is_nonzero
    }

    pub fn ct_select(mask: u64, a: &Self, b: &Self) -> Self {
        Self([
            (a.0[0] & mask) | (b.0[0] & !mask),
            (a.0[1] & mask) | (b.0[1] & !mask),
            (a.0[2] & mask) | (b.0[2] & !mask),
            (a.0[3] & mask) | (b.0[3] & !mask),
        ])
    }

    pub fn ct_eq(&self, other: &Self) -> bool {
        let mut diff = 0u64;
        diff |= self.0[0] ^ other.0[0];
        diff |= self.0[1] ^ other.0[1];
        diff |= self.0[2] ^ other.0[2];
        diff |= self.0[3] ^ other.0[3];
        diff == 0
    }

    pub fn negate(&self) -> Self {
        let mut result = [0u64; 4];
        let mut borrow = 0i128;

        for (out, (a, b)) in result.iter_mut().zip(Self::N.iter().zip(self.0.iter())) {

            borrow += *a as i128 - *b as i128;

            if borrow < 0 {

                *out = (borrow + (1i128 << 64)) as u64;

                borrow = -1;

            } else {

                *out = borrow as u64;

                borrow = 0;

            }

        }

        Self(result)
    }
}
