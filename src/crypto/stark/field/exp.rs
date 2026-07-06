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

//! Field exponentiation and inversion.

use super::element::{Fp, P};

impl Fp {
    #[inline]
    pub fn square(self) -> Fp {
        self * self
    }

    /// Exponentiation by square and multiply.
    pub fn pow(self, mut exp: u64) -> Fp {
        let mut base = self;
        let mut acc = Fp::ONE;
        while exp != 0 {
            if exp & 1 == 1 {
                acc = acc * base;
            }
            base = base.square();
            exp >>= 1;
        }
        acc
    }

    /// The multiplicative inverse via Fermat's little theorem, `a^(p-2)`.
    /// Returns `ZERO` for `ZERO`, which has no inverse; callers must exclude it.
    pub fn inv(self) -> Fp {
        self.pow(P - 2)
    }
}
