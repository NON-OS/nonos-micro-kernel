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

//! Evaluation of a polynomial given in coefficient form, low degree first.

use super::super::field::Fp;

/// Evaluate `coeffs[0] + coeffs[1] x + coeffs[2] x^2 + ...` at `x` by Horner's
/// method. An empty coefficient list is the zero polynomial.
pub fn eval(coeffs: &[Fp], x: Fp) -> Fp {
    let mut acc = Fp::ZERO;
    for &c in coeffs.iter().rev() {
        acc = acc * x + c;
    }
    acc
}
