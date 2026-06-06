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

extern crate alloc;
use super::super::BigUint;
use alloc::vec::Vec;

impl BigUint {
    pub(crate) fn montgomery_inverse(m0: u64) -> Option<u64> {
        if m0 & 1 != 1 {
            return None;
        }

        let mut y = 1u64;
        for _ in 0..6 {
            y = y.wrapping_mul(2u64.wrapping_sub(m0.wrapping_mul(y)));
        }
        Some(y.wrapping_neg())
    }

    pub(crate) fn montgomery_reduce(t: &Self, modulus: &Self, m_inv: u64) -> Self {
        let n = modulus.limbs.len();
        let mut a = t.limbs.clone();

        while a.len() < 2 * n {
            a.push(0);
        }

        for i in 0..n {
            let u = a[i].wrapping_mul(m_inv);

            let mut carry = 0u128;
            for j in 0..n {
                let sum = (a[i + j] as u128) + (u as u128) * (modulus.limbs[j] as u128) + carry;
                a[i + j] = sum as u64;
                carry = sum >> 64;
            }

            let mut k = i + n;
            while carry != 0 && k < a.len() {
                let sum = (a[k] as u128) + carry;
                a[k] = sum as u64;
                carry = sum >> 64;
                k += 1;
            }
            if carry != 0 {
                a.push(carry as u64);
            }
        }

        let result_limbs: Vec<u64> = a[n..].to_vec();
        let mut result = BigUint::normalize(result_limbs);

        if &result >= modulus {
            result = &result - modulus;
        }

        result
    }
}
