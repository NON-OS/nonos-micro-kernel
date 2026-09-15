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
    pub fn add(&self, other: &Self) -> Self {
        let mut result = [0u64; 4];
        let mut carry = 0u128;

        for (out, (a, b)) in result.iter_mut().zip(self.0.iter().zip(other.0.iter())) {

            carry += *a as u128 + *b as u128;

            *out = carry as u64;

            carry >>= 64;

        }

        let mut fold = carry * 0x1000003D1u128;
        for limb in result.iter_mut() {
            fold += *limb as u128;
            *limb = fold as u64;
            fold >>= 64;
        }
        let mut fold2 = fold * 0x1000003D1u128;
        for limb in result.iter_mut() {
            fold2 += *limb as u128;
            *limb = fold2 as u64;
            fold2 >>= 64;
        }

        let mut res = Self(result);
        res.reduce();
        res
    }

    pub fn sub(&self, other: &Self) -> Self {
        let mut result = [0u64; 4];
        let mut borrow = 0i128;

        for (out, (a, b)) in result.iter_mut().zip(self.0.iter().zip(other.0.iter())) {

            borrow += *a as i128 - *b as i128;

            if borrow < 0 {

                *out = (borrow + (1i128 << 64)) as u64;

                borrow = -1;

            } else {

                *out = borrow as u64;

                borrow = 0;

            }

        }

        let mut res = Self(result);
        if borrow < 0 {
            let mut carry = 0u128;
            for (out, c) in res.0.iter_mut().zip(Self::P.iter()) {
                carry += *out as u128 + *c as u128;
                *out = carry as u64;
                carry >>= 64;
            }
        }
        res
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut t = [0u128; 8];

        for i in 0..4 {
            let a = self.0[i] as u128;
            let mut carry = 0u128;
            for j in 0..4 {
                let sum = a * other.0[j] as u128 + t[i + j] + carry;
                t[i + j] = sum & 0xFFFFFFFFFFFFFFFF;
                carry = sum >> 64;
            }
            t[i + 4] = carry;
        }

        let mut result = [0u64; 4];
        for i in 0..4 {
            result[i] = t[i] as u64;
        }

        Self(result).reduce_wide(&t)
    }

    pub fn square(&self) -> Self {
        self.mul(self)
    }
}
