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

use libm::{acos, asin, atan, cos, exp, fabs, log, log10, sin, tan};

use super::convert::{from_f64, to_f64};
use super::factorial::factorial;
use super::kind::SciFn;
use crate::calc::fixed::Fixed;
use crate::calc::state::ErrorKind;

const TAN_CEILING: f64 = 1.0e12;

fn unit(x: f64) -> Result<f64, ErrorKind> {
    if !(-1.0..=1.0).contains(&x) {
        return Err(ErrorKind::DomainError);
    }
    Ok(x)
}

fn positive(x: f64) -> Result<f64, ErrorKind> {
    if x <= 0.0 {
        return Err(ErrorKind::DomainError);
    }
    Ok(x)
}

fn tangent(x: f64) -> Result<f64, ErrorKind> {
    let t = tan(x);
    if !t.is_finite() || fabs(t) > TAN_CEILING {
        return Err(ErrorKind::DomainError);
    }
    Ok(t)
}

pub fn apply(f: SciFn, value: Fixed) -> Result<Fixed, ErrorKind> {
    let x = to_f64(value);
    match f {
        SciFn::Factorial => factorial(value),
        SciFn::Sin => from_f64(sin(x)),
        SciFn::Cos => from_f64(cos(x)),
        SciFn::Tan => from_f64(tangent(x)?),
        SciFn::Asin => from_f64(asin(unit(x)?)),
        SciFn::Acos => from_f64(acos(unit(x)?)),
        SciFn::Atan => from_f64(atan(x)),
        SciFn::Ln => from_f64(log(positive(x)?)),
        SciFn::Log10 => from_f64(log10(positive(x)?)),
        SciFn::Exp => from_f64(exp(x)),
    }
}
