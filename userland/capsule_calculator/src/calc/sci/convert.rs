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

const LIMIT: f64 = 1.0e30;

pub fn to_f64(value: Fixed) -> f64 {
    (value as f64) / (FRAC as f64)
}

pub fn from_f64(value: f64) -> Result<Fixed, ErrorKind> {
    if value.is_nan() {
        return Err(ErrorKind::DomainError);
    }
    if !value.is_finite() {
        return Err(ErrorKind::Overflow);
    }
    let scaled = value * (FRAC as f64);
    if scaled > LIMIT || scaled < -LIMIT {
        return Err(ErrorKind::Overflow);
    }
    Ok(scaled as Fixed)
}
