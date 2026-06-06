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

use super::super::{BigUint, LIMB_BITS};

impl BigUint {
    pub fn mod_pow(&self, exp: &Self, modulus: &Self) -> Option<Self> {
        if modulus.is_zero() {
            return None;
        }
        if modulus.is_one() {
            return Some(Self::zero());
        }
        if exp.is_zero() {
            return Some(Self::one());
        }

        if modulus.is_odd() && modulus.bits() >= 64 {
            return self.mod_pow_montgomery_ct(exp, modulus);
        }

        self.mod_pow_ct(exp, modulus)
    }

    pub(crate) fn mod_pow_ct(&self, exp: &Self, modulus: &Self) -> Option<Self> {
        let bit_len = exp.bits();
        if bit_len == 0 {
            return Some(Self::one());
        }

        let mut result = Self::one();
        let mut base = self % modulus;

        for i in 0..bit_len {
            let mul_result = &(&result * &base) % modulus;
            let bit = exp.bit_ct(i);
            let mask = 0u64.wrapping_sub(bit);
            result = Self::ct_select(mask, &mul_result, &result);
            base = base.square() % modulus;
        }

        Some(result)
    }

    pub(crate) fn mod_pow_montgomery_ct(&self, exp: &Self, modulus: &Self) -> Option<Self> {
        if !modulus.is_odd() || modulus.is_zero() {
            return None;
        }

        let n = modulus.limbs.len();
        let r_bits = n * LIMB_BITS;

        let r = Self::one().shl_bits(r_bits) % modulus;
        let r2 = r.square() % modulus;
        let m_inv = Self::montgomery_inverse(modulus.limbs[0])?;

        let mut base = self % modulus;
        base = Self::montgomery_reduce(&(&base * &r2), modulus, m_inv);

        let mut r0 = r.clone();
        let mut r1 = base;

        let bit_len = exp.bits();
        for i in (0..bit_len).rev() {
            let bit = exp.bit_ct(i);

            let r0_times_r1 = Self::montgomery_reduce(&(&r0 * &r1), modulus, m_inv);
            let r0_squared = Self::montgomery_reduce(&r0.square(), modulus, m_inv);
            let r1_squared = Self::montgomery_reduce(&r1.square(), modulus, m_inv);

            let mask = 0u64.wrapping_sub(bit);
            r0 = Self::ct_select(mask, &r0_times_r1, &r0_squared);
            r1 = Self::ct_select(mask, &r1_squared, &r0_times_r1);
        }

        Some(Self::montgomery_reduce(&r0, modulus, m_inv))
    }
}
