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

use super::fixed::{Fixed, FRAC};
use super::state::ErrorKind;

pub fn square(value: Fixed) -> Result<Fixed, ErrorKind> {
    let prod = value.checked_mul(value).ok_or(ErrorKind::Overflow)?;
    Ok(prod / FRAC)
}

pub fn reciprocal(value: Fixed) -> Result<Fixed, ErrorKind> {
    if value == 0 {
        return Err(ErrorKind::DivByZero);
    }
    let scaled = (FRAC as i128).checked_mul(FRAC).ok_or(ErrorKind::Overflow)?;
    Ok(scaled / value)
}

pub fn sqrt(value: Fixed) -> Result<Fixed, ErrorKind> {
    if value < 0 {
        return Err(ErrorKind::DomainError);
    }
    if value == 0 {
        return Ok(0);
    }
    let scaled = value.checked_mul(FRAC).ok_or(ErrorKind::Overflow)?;
    Ok(integer_sqrt_u128(scaled as u128) as Fixed)
}

fn integer_sqrt_u128(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}
