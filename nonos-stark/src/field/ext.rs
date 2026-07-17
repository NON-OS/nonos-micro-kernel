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

//! The degree-2 extension Fp2 = Fp[X]/(X^2 - 7), a field of ~2^128 elements.
//!
//! FRI and DEEP challenges are drawn from this field, not the 64-bit base. The
//! low-degree test's soundness error is (degree / |challenge field|): over the
//! base field it is capped near 2^-64, which does not guard funds; over Fp2 it
//! drops to ~2^-128. The bound is proved in the Lean tower,
//! `Nonos.Stark.Soundness.the_soundness_error_is_below_the_degree`, so drawing
//! challenges here is what that theorem's `cs` must be to be sound.
//!
//! The non-residue is W = 7: it is a quadratic non-residue in Goldilocks, so
//! `X^2 - 7` is irreducible and the quotient is a field. This is the same
//! non-residue Plonky2 uses for the Goldilocks quadratic extension.

use super::element::Fp;

/// The non-residue: `X^2 = W`.
const W: Fp = Fp::from_u64(7);

/// An element `c0 + c1*X` of Fp2, held as its coefficient pair.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fp2 {
    pub c0: Fp,
    pub c1: Fp,
}

impl Fp2 {
    pub const ZERO: Fp2 = Fp2 { c0: Fp::ZERO, c1: Fp::ZERO };
    pub const ONE: Fp2 = Fp2 { c0: Fp::ONE, c1: Fp::ZERO };

    /// Build from the two coefficients.
    #[inline]
    pub const fn new(c0: Fp, c1: Fp) -> Fp2 {
        Fp2 { c0, c1 }
    }

    /// Embed a base element as `a + 0*X`.
    #[inline]
    pub const fn from_base(a: Fp) -> Fp2 {
        Fp2 { c0: a, c1: Fp::ZERO }
    }

    /// The conjugate `c0 - c1*X`, the image under the nontrivial automorphism.
    #[inline]
    pub fn conjugate(self) -> Fp2 {
        Fp2 { c0: self.c0, c1: -self.c1 }
    }

    /// The norm down to the base field, `N = c0^2 - W*c1^2 = (a+bX)(a-bX)`. It is
    /// zero only for the zero element, since `X^2 - 7` has no root in the base.
    #[inline]
    pub fn norm(self) -> Fp {
        self.c0 * self.c0 - W * (self.c1 * self.c1)
    }

    #[inline]
    pub fn square(self) -> Fp2 {
        self * self
    }

    /// Exponentiation by square and multiply, as in the base field.
    pub fn pow(self, mut exp: u64) -> Fp2 {
        let mut base = self;
        let mut acc = Fp2::ONE;
        while exp != 0 {
            if exp & 1 == 1 {
                acc = acc * base;
            }
            base = base.square();
            exp >>= 1;
        }
        acc
    }

    /// Scale by a base-field element, `s * (c0 + c1 X) = s c0 + s c1 X`. Cheaper
    /// than a full extension multiply, used where one operand is in the base field,
    /// as in folding a base-field codeword with an extension challenge.
    #[inline]
    pub fn mul_base(self, s: Fp) -> Fp2 {
        Fp2 { c0: self.c0 * s, c1: self.c1 * s }
    }

    /// The multiplicative inverse, `(a - bX) / N`. Returns `ZERO` for `ZERO`,
    /// which has no inverse; callers must exclude it, as in the base field.
    pub fn inv(self) -> Fp2 {
        if self == Fp2::ZERO {
            return Fp2::ZERO;
        }
        let n_inv = self.norm().inv();
        let conj = self.conjugate();
        Fp2 { c0: conj.c0 * n_inv, c1: conj.c1 * n_inv }
    }
}

impl core::ops::Add for Fp2 {
    type Output = Fp2;

    #[inline]
    fn add(self, o: Fp2) -> Fp2 {
        Fp2 { c0: self.c0 + o.c0, c1: self.c1 + o.c1 }
    }
}

impl core::ops::Sub for Fp2 {
    type Output = Fp2;

    #[inline]
    fn sub(self, o: Fp2) -> Fp2 {
        Fp2 { c0: self.c0 - o.c0, c1: self.c1 - o.c1 }
    }
}

impl core::ops::Neg for Fp2 {
    type Output = Fp2;

    #[inline]
    fn neg(self) -> Fp2 {
        Fp2 { c0: -self.c0, c1: -self.c1 }
    }
}

impl core::ops::Mul for Fp2 {
    type Output = Fp2;

    /// `(a+bX)(c+dX) = (ac + W*bd) + (ad + bc)X`, since `X^2 = W`.
    #[inline]
    fn mul(self, o: Fp2) -> Fp2 {
        let ac = self.c0 * o.c0;
        let bd = self.c1 * o.c1;
        let ad = self.c0 * o.c1;
        let bc = self.c1 * o.c0;
        Fp2 { c0: ac + W * bd, c1: ad + bc }
    }
}
