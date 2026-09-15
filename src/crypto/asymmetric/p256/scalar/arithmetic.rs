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

pub(crate) fn reduce(s: &mut Scalar) {
    let mut borrow = 0i128;
    let mut temp = [0u64; 4];

    for (out, (a, b)) in temp.iter_mut().zip(s.0.iter().zip(Scalar::N.iter())) {

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
    for (out, t) in s.0.iter_mut().zip(temp.iter()) {
        *out = (*t & mask) | (*out & !mask);
    }
}

fn reduce_wide(wide: &[u128; 8]) -> Scalar {
    const C: [u64; 4] =
        [0x0C46353D039CDAAF, 0x4319055258E8617B, 0x0000000000000000, 0x00000000FFFFFFFF];

    const N: [u64; 4] =
        [0xF3B9CAC2FC632551, 0xBCE6FAADA7179E84, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFF00000000];

    let mut val = [0u64; 9];
    for i in 0..8 {
        val[i] = wide[i] as u64;
    }

    for _ in 0..10 {
        let low = [val[0], val[1], val[2], val[3]];
        let high = [val[4], val[5], val[6], val[7], val[8]];

        let mut hc = [0u64; 9];
        for (i, &h) in high.iter().enumerate() {
            let mut carry = 0u128;
            for (j, &c) in C.iter().enumerate() {
                let idx = i + j;
                if idx < 9 {
                    let product = (h as u128) * (c as u128);
                    let sum = (hc[idx] as u128) + product + carry;
                    hc[idx] = sum as u64;
                    carry = sum >> 64;
                }
            }
            for slot in hc.iter_mut().take(i + 6).skip(i + 4) {
                let sum = (*slot as u128) + carry;
                *slot = sum as u64;
                carry = sum >> 64;
            }
        }

        let mut carry = 0u128;
        for i in 0..4 {
            carry += (low[i] as u128) + (hc[i] as u128);
            val[i] = carry as u64;
            carry >>= 64;
        }
        for i in 4..9 {
            carry += hc[i] as u128;
            val[i] = carry as u64;
            carry >>= 64;
        }
    }

    let mut result = Scalar([val[0], val[1], val[2], val[3]]);

    for _ in 0..3 {
        let mut temp = [0u64; 4];
        let mut borrow = 0i128;

        for (out, (a, b)) in temp.iter_mut().zip(result.0.iter().zip(N.iter())) {

            borrow += *a as i128 - *b as i128;

            if borrow < 0 {

                *out = ((1i128 << 64) + borrow) as u64;

                borrow = -1;

            } else {

                *out = borrow as u64;

                borrow = 0;

            }

        }

        let no_borrow = ((borrow >> 127) & 1) as u64;
        let mask = no_borrow.wrapping_sub(1);
        for (out, t) in result.0.iter_mut().zip(temp.iter()) {
            *out = (*t & mask) | (*out & !mask);
        }
    }

    result
}

impl Scalar {
    pub fn add(&self, other: &Self) -> Self {
        let mut result = [0u64; 4];
        let mut carry = 0u128;

        for (out, (a, b)) in result.iter_mut().zip(self.0.iter().zip(other.0.iter())) {

            carry += *a as u128 + *b as u128;

            *out = carry as u64;

            carry >>= 64;

        }

        let mut res = Self(result);

        if carry != 0 {
            const C: [u64; 4] =
                [0x0C46353D039CDAAF, 0x4319055258E8617B, 0x0000000000000000, 0x00000000FFFFFFFF];
            let mut add_carry = 0u128;
            for (out, c) in res.0.iter_mut().zip(C.iter()) {
                add_carry += *out as u128 + *c as u128;
                *out = add_carry as u64;
                add_carry >>= 64;
            }
        }

        reduce(&mut res);
        res
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut result = [0u64; 8];

        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                let product = (self.0[i] as u128) * (other.0[j] as u128);
                let sum = (result[i + j] as u128) + product + carry;
                result[i + j] = sum as u64;
                carry = sum >> 64;
            }
            let mut k = i + 4;
            while carry != 0 && k < 8 {
                let sum = (result[k] as u128) + carry;
                result[k] = sum as u64;
                carry = sum >> 64;
                k += 1;
            }
        }

        let wide: [u128; 8] = [
            result[0] as u128,
            result[1] as u128,
            result[2] as u128,
            result[3] as u128,
            result[4] as u128,
            result[5] as u128,
            result[6] as u128,
            result[7] as u128,
        ];

        reduce_wide(&wide)
    }
}
