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

use crate::calc::fixed::{Fixed, FRAC};
use crate::calc::state::ErrorKind;

const MAX_N: Fixed = 20;

pub fn factorial(value: Fixed) -> Result<Fixed, ErrorKind> {
    if value < 0 || value % FRAC != 0 {
        return Err(ErrorKind::DomainError);
    }
    let n = value / FRAC;
    if n > MAX_N {
        return Err(ErrorKind::DomainError);
    }
    let mut acc: Fixed = 1;
    let mut k: Fixed = 2;
    while k <= n {
        acc = acc.checked_mul(k).ok_or(ErrorKind::Overflow)?;
        k += 1;
    }
    acc.checked_mul(FRAC).ok_or(ErrorKind::Overflow)
}
