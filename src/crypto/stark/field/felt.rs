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

//! The field abstraction an AIR constraint is written over, so one transition
//! serves both the base field (the composition over the coset) and the extension
//! (the out-of-domain evaluation a money-grade STARK samples at `z in Fp2`). Base
//! constants, round constants, and periodic values enter through `from_base`.

use super::element::Fp;
use super::ext::Fp2;

/// A field a constraint can be evaluated over: the base `Fp` or its extension
/// `Fp2`. Only the operations a transition and the composition need are required.
pub trait Felt:
    Copy
    + PartialEq
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;

    /// Embed a base-field element, so shared constants live in one place.
    fn from_base(x: Fp) -> Self;

    /// Exponentiation, for domain vanishing polynomials.
    fn pow(self, exp: u64) -> Self;

    /// Multiplicative inverse; the caller guarantees a nonzero argument, as the
    /// off-domain evaluation points do.
    fn inv(self) -> Self;
}

impl Felt for Fp {
    const ZERO: Fp = Fp::ZERO;
    const ONE: Fp = Fp::ONE;

    #[inline]
    fn from_base(x: Fp) -> Fp {
        x
    }

    #[inline]
    fn pow(self, exp: u64) -> Fp {
        Fp::pow(self, exp)
    }

    #[inline]
    fn inv(self) -> Fp {
        Fp::inv(self)
    }
}

impl Felt for Fp2 {
    const ZERO: Fp2 = Fp2::ZERO;
    const ONE: Fp2 = Fp2::ONE;

    #[inline]
    fn from_base(x: Fp) -> Fp2 {
        Fp2::from_base(x)
    }

    #[inline]
    fn pow(self, exp: u64) -> Fp2 {
        Fp2::pow(self, exp)
    }

    #[inline]
    fn inv(self) -> Fp2 {
        Fp2::inv(self)
    }
}
