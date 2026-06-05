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

use alloc::vec;
use core::cmp::Ordering;
use core::ops::{Div, Rem};

use super::BigUint;

impl BigUint {
    pub fn div_rem(&self, divisor: &Self) -> Option<(Self, Self)> {
        if divisor.is_zero() {
            return None;
        }

        if self.is_zero() {
            return Some((Self::zero(), Self::zero()));
        }

        match self.cmp(divisor) {
            Ordering::Less => return Some((Self::zero(), self.clone())),
            Ordering::Equal => return Some((Self::one(), Self::zero())),
            Ordering::Greater => {}
        }

        if divisor.limbs.len() == 1 {
            return self.div_rem_u64(divisor.limbs[0]);
        }

        Some(self.div_rem_knuth(divisor))
    }

    pub(crate) fn div_rem_u64(&self, divisor: u64) -> Option<(Self, Self)> {
        if divisor == 0 {
            return None;
        }

        let mut quotient = vec![0u64; self.limbs.len()];
        let mut remainder = 0u128;

        for i in (0..self.limbs.len()).rev() {
            let dividend = (remainder << 64) | (self.limbs[i] as u128);
            quotient[i] = (dividend / (divisor as u128)) as u64;
            remainder = dividend % (divisor as u128);
        }

        Some((BigUint::normalize(quotient), BigUint::from_u64(remainder as u64)))
    }

    pub(crate) fn div_rem_knuth(&self, divisor: &Self) -> (Self, Self) {
        let n = divisor.limbs.len();
        let m = self.limbs.len() - n;

        let shift = divisor.limbs[n - 1].leading_zeros();
        let mut u = self.shl_bits(shift as usize);
        let v = divisor.shl_bits(shift as usize);

        while u.limbs.len() <= m + n {
            u.limbs.push(0);
        }

        let mut q = vec![0u64; m + 1];

        for j in (0..=m).rev() {
            let u_hi = ((u.limbs[j + n] as u128) << 64) | (u.limbs[j + n - 1] as u128);
            let mut qhat = u_hi / (v.limbs[n - 1] as u128);
            let mut rhat = u_hi % (v.limbs[n - 1] as u128);

            while qhat >= (1u128 << 64)
                || (n >= 2
                    && qhat * (v.limbs[n - 2] as u128)
                        > ((rhat << 64) | (u.limbs[j + n - 2] as u128)))
            {
                qhat -= 1;
                rhat += v.limbs[n - 1] as u128;
                if rhat >= (1u128 << 64) {
                    break;
                }
            }

            let mut borrow = 0i128;
            for i in 0..n {
                let product = (qhat as u128) * (v.limbs[i] as u128);
                let sub = (u.limbs[j + i] as i128) - (product as u64 as i128) - borrow;
                u.limbs[j + i] = sub as u64;
                borrow = (product >> 64) as i128 - (sub >> 64);
            }
            let sub = (u.limbs[j + n] as i128) - borrow;
            u.limbs[j + n] = sub as u64;

            q[j] = qhat as u64;

            if sub < 0 {
                q[j] -= 1;
                let mut carry = 0u64;
                for i in 0..n {
                    let sum = (u.limbs[j + i] as u128) + (v.limbs[i] as u128) + (carry as u128);
                    u.limbs[j + i] = sum as u64;
                    carry = (sum >> 64) as u64;
                }
                u.limbs[j + n] = u.limbs[j + n].wrapping_add(carry);
            }
        }

        u.limbs.truncate(n);
        let remainder = BigUint::normalize(u.limbs.clone()).shr_bits(shift as usize);

        (BigUint::normalize(q), remainder)
    }
}

impl Div<&BigUint> for &BigUint {
    type Output = BigUint;
    fn div(self, other: &BigUint) -> BigUint {
        self.div_rem(other).map(|(q, _)| q).unwrap_or_else(BigUint::zero)
    }
}

impl Div<BigUint> for BigUint {
    type Output = BigUint;
    fn div(self, other: BigUint) -> BigUint {
        &self / &other
    }
}

impl Div<&BigUint> for BigUint {
    type Output = BigUint;
    fn div(self, other: &BigUint) -> BigUint {
        &self / other
    }
}

impl Div<BigUint> for &BigUint {
    type Output = BigUint;
    fn div(self, other: BigUint) -> BigUint {
        self / &other
    }
}

impl Rem<&BigUint> for &BigUint {
    type Output = BigUint;
    fn rem(self, other: &BigUint) -> BigUint {
        self.div_rem(other).map(|(_, r)| r).unwrap_or_else(|| self.clone())
    }
}

impl Rem<BigUint> for BigUint {
    type Output = BigUint;
    fn rem(self, other: BigUint) -> BigUint {
        &self % &other
    }
}

impl Rem<&BigUint> for BigUint {
    type Output = BigUint;
    fn rem(self, other: &BigUint) -> BigUint {
        &self % other
    }
}

impl Rem<BigUint> for &BigUint {
    type Output = BigUint;
    fn rem(self, other: BigUint) -> BigUint {
        self % &other
    }
}
