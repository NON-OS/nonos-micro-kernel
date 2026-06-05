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

use super::super::BigUint;
use core::ops::Sub;

impl Sub<&BigUint> for &BigUint {
    type Output = BigUint;

    fn sub(self, other: &BigUint) -> BigUint {
        if self < other {
            return BigUint::zero();
        }

        let mut result = self.limbs.clone();
        let mut borrow = 0i128;

        for i in 0..result.len() {
            let a = result[i] as i128;
            let b = other.limbs.get(i).copied().unwrap_or(0) as i128;
            let diff = a - b - borrow;

            if diff < 0 {
                result[i] = (diff + (1i128 << 64)) as u64;
                borrow = 1;
            } else {
                result[i] = diff as u64;
                borrow = 0;
            }
        }

        if borrow != 0 {
            return BigUint::zero();
        }
        BigUint::normalize(result)
    }
}

impl Sub<BigUint> for BigUint {
    type Output = BigUint;
    fn sub(self, other: BigUint) -> BigUint {
        &self - &other
    }
}

impl Sub<&BigUint> for BigUint {
    type Output = BigUint;
    fn sub(self, other: &BigUint) -> BigUint {
        &self - other
    }
}

impl Sub<BigUint> for &BigUint {
    type Output = BigUint;
    fn sub(self, other: BigUint) -> BigUint {
        self - &other
    }
}

impl BigUint {
    pub fn sub_u64(&self, val: u64) -> Option<Self> {
        if self.limbs.len() == 1 && self.limbs[0] < val {
            return None;
        }

        let mut result = self.limbs.clone();
        let mut borrow = val as u128;
        let mut i = 0;

        while borrow != 0 && i < result.len() {
            if (result[i] as u128) >= borrow {
                result[i] -= borrow as u64;
                borrow = 0;
            } else {
                let diff = ((1u128 << 64) + (result[i] as u128)) - borrow;
                result[i] = diff as u64;
                borrow = 1;
            }
            i += 1;
        }

        if borrow != 0 {
            return None;
        }

        Some(BigUint::normalize(result))
    }

    pub fn saturating_sub(&self, other: &Self) -> Self {
        if self >= other {
            self - other
        } else {
            Self::zero()
        }
    }
}
