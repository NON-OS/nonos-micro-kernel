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
use super::sci::power;
use super::state::ErrorKind;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Op {
    None,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub fn apply(a: Fixed, b: Fixed, op: Op) -> Result<Fixed, ErrorKind> {
    match op {
        Op::None => Ok(b),
        Op::Add => a.checked_add(b).ok_or(ErrorKind::Overflow),
        Op::Sub => a.checked_sub(b).ok_or(ErrorKind::Overflow),
        Op::Mul => {
            let prod = a.checked_mul(b).ok_or(ErrorKind::Overflow)?;
            Ok(prod / FRAC)
        }
        Op::Div => {
            if b == 0 {
                return Err(ErrorKind::DivByZero);
            }
            let scaled = a.checked_mul(FRAC).ok_or(ErrorKind::Overflow)?;
            Ok(scaled / b)
        }
        Op::Pow => power(a, b),
    }
}
