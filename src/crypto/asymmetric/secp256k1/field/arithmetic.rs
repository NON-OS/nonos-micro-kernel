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

        for i in 0..4 {
            carry += self.0[i] as u128 + other.0[i] as u128;
            result[i] = carry as u64;
            carry >>= 64;
        }

        let mut fold = carry * 0x1000003D1u128;
        for i in 0..4 {
            fold += result[i] as u128;
            result[i] = fold as u64;
            fold >>= 64;
        }
        let mut fold2 = fold * 0x1000003D1u128;
        for i in 0..4 {
            fold2 += result[i] as u128;
            result[i] = fold2 as u64;
            fold2 >>= 64;
        }

        let mut res = Self(result);
        res.reduce();
        res
    }

    pub fn sub(&self, other: &Self) -> Self {
        let mut result = [0u64; 4];
        let mut borrow = 0i128;

        for i in 0..4 {
            borrow += self.0[i] as i128 - other.0[i] as i128;
            if borrow < 0 {
                result[i] = (borrow + (1i128 << 64)) as u64;
                borrow = -1;
            } else {
                result[i] = borrow as u64;
                borrow = 0;
            }
        }

        let mut res = Self(result);
        if borrow < 0 {
            let mut carry = 0u128;
            for i in 0..4 {
                carry += res.0[i] as u128 + Self::P[i] as u128;
                res.0[i] = carry as u64;
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
