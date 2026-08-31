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

use libm::{pow, trunc};

use super::convert::{from_f64, to_f64};
use crate::calc::fixed::Fixed;
use crate::calc::state::ErrorKind;

pub fn power(base: Fixed, exponent: Fixed) -> Result<Fixed, ErrorKind> {
    let b = to_f64(base);
    let e = to_f64(exponent);
    if b == 0.0 && e < 0.0 {
        return Err(ErrorKind::DivByZero);
    }
    if b < 0.0 && e != trunc(e) {
        return Err(ErrorKind::DomainError);
    }
    from_f64(pow(b, e))
}
