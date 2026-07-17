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

//! The evaluation domain: a multiplicative subgroup of size a power of two.

use super::super::field::{Fp, P};

/// A multiplicative generator of the Goldilocks field. Every nonzero element is
/// a power of it, so `GENERATOR^((P-1)/2^k)` has order exactly `2^k`.
const GENERATOR: u64 = 7;

/// A primitive `2^log_n`-th root of unity. The returned `omega` generates the
/// size-`2^log_n` subgroup used as the FRI evaluation domain: `omega^(2^log_n)`
/// is one and `omega^(2^(log_n-1))` is minus one, so the domain is closed under
/// negation, which is what the folding step requires. Valid for `log_n <= 32`,
/// the two-adicity of this field.
pub fn root_of_unity(log_n: u32) -> Fp {
    Fp::from_u64(GENERATOR).pow((P - 1) >> log_n)
}
