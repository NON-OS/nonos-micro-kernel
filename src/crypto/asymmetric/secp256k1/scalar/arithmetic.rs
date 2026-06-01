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

    for i in 0..4 {
        borrow += s.0[i] as i128 - Scalar::N[i] as i128;
        if borrow < 0 {
            temp[i] = (borrow + (1i128 << 64)) as u64;
            borrow = -1;
        } else {
            temp[i] = borrow as u64;
            borrow = 0;
        }
    }

    let no_borrow = ((borrow >> 127) & 1) as u64;
    let mask = no_borrow.wrapping_sub(1);
    for i in 0..4 {
        s.0[i] = (temp[i] & mask) | (s.0[i] & !mask);
    }
}

fn reduce_wide(wide: &[u128; 8]) -> Scalar {
    const R: [u64; 4] =
        [0x402DA1732FC9BEBF, 0x4551231950B75FC4, 0x0000000000000001, 0x0000000000000000];

    let mut limbs = [0u64; 8];
    for i in 0..8 {
        limbs[i] = wide[i] as u64;
    }

    while limbs[4] | limbs[5] | limbs[6] | limbs[7] != 0 {
        let mut acc = [0u64; 8];
        acc[0] = limbs[0];
        acc[1] = limbs[1];
        acc[2] = limbs[2];
        acc[3] = limbs[3];

        for i in 0..4 {
            let mut carry = 0u128;
            for j in 0..4 {
                let p = acc[i + j] as u128 + limbs[4 + i] as u128 * R[j] as u128 + carry;
                acc[i + j] = p as u64;
                carry = p >> 64;
            }
            let mut q = i + 4;
            while carry != 0 && q < 8 {
                let p = acc[q] as u128 + carry;
                acc[q] = p as u64;
                carry = p >> 64;
                q += 1;
            }
        }
        limbs = acc;
    }

    let mut res = Scalar([limbs[0], limbs[1], limbs[2], limbs[3]]);
    reduce(&mut res);
    reduce(&mut res);
    res
}

impl Scalar {
    pub fn add(&self, other: &Self) -> Self {
        let mut result = [0u64; 4];
        let mut carry = 0u128;

        for i in 0..4 {
            carry += self.0[i] as u128 + other.0[i] as u128;
            result[i] = carry as u64;
            carry >>= 64;
        }

        const RR: [u64; 4] =
            [0x402DA1732FC9BEBF, 0x4551231950B75FC4, 0x0000000000000001, 0x0000000000000000];
        let mut fold = 0u128;
        for i in 0..4 {
            fold += result[i] as u128 + carry * RR[i] as u128;
            result[i] = fold as u64;
            fold >>= 64;
        }
        let mut fold2 = 0u128;
        for i in 0..4 {
            fold2 += result[i] as u128 + fold * RR[i] as u128;
            result[i] = fold2 as u64;
            fold2 >>= 64;
        }

        let mut res = Self(result);
        reduce(&mut res);
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

        reduce_wide(&t)
    }
}
