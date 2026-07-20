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

//! The field ring operations. Multiplication reduces through a `u128` modulo;
//! the optimized special-form reduction is a later refinement held to the same
//! specification the property proofs fix.

use super::element::{Fp, EPSILON, P};

impl core::ops::Add for Fp {
    type Output = Fp;

    #[inline]
    fn add(self, other: Fp) -> Fp {
        let (sum, over) = self.0.overflowing_add(other.0);
        // a + b lies in [0, 2P). On overflow past 2^64 the field result is
        // sum + (2^64 - P) = sum + EPSILON, already canonical. Otherwise one
        // conditional subtraction canonicalizes.
        let mut r = sum;
        if over {
            r = r.wrapping_add(EPSILON);
        } else if r >= P {
            r -= P;
        }
        Fp(r)
    }
}

impl core::ops::Sub for Fp {
    type Output = Fp;

    #[inline]
    fn sub(self, other: Fp) -> Fp {
        let (diff, borrow) = self.0.overflowing_sub(other.0);
        // On borrow the wrapped value is a - b + 2^64; the field result a - b + P
        // is that minus (2^64 - P) = minus EPSILON.
        let r = if borrow { diff.wrapping_sub(EPSILON) } else { diff };
        Fp(r)
    }
}

impl core::ops::Neg for Fp {
    type Output = Fp;

    #[inline]
    fn neg(self) -> Fp {
        if self.0 == 0 {
            Fp(0)
        } else {
            Fp(P - self.0)
        }
    }
}

impl core::ops::Mul for Fp {
    type Output = Fp;

    #[inline]
    fn mul(self, other: Fp) -> Fp {
        let product = (self.0 as u128) * (other.0 as u128);
        Fp((product % (P as u128)) as u64)
    }
}
