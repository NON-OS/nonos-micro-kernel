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

use super::temp::{CELSIUS, FAHRENHEIT, KELVIN};
use super::units::{list, Category};
use crate::calc::fixed::{Fixed, FRAC};

const K_OFF: Fixed = 27_315 * (FRAC / 100);
const F_OFF: Fixed = 32 * FRAC;

fn to_celsius(value: Fixed, unit: usize) -> Option<Fixed> {
    match unit {
        CELSIUS => Some(value),
        FAHRENHEIT => Some((value - F_OFF) * 5 / 9),
        KELVIN => Some(value - K_OFF),
        _ => None,
    }
}

fn from_celsius(celsius: Fixed, unit: usize) -> Option<Fixed> {
    match unit {
        CELSIUS => Some(celsius),
        FAHRENHEIT => Some(celsius * 9 / 5 + F_OFF),
        KELVIN => Some(celsius + K_OFF),
        _ => None,
    }
}

pub fn convert(cat: Category, from: usize, to: usize, value: Fixed) -> Option<Fixed> {
    if cat == Category::Temperature {
        return from_celsius(to_celsius(value, from)?, to);
    }
    let units = list(cat);
    let src = units.get(from)?;
    let dst = units.get(to)?;
    let num = src.num.checked_mul(dst.den)?;
    let den = src.den.checked_mul(dst.num)?;
    if den == 0 {
        return None;
    }
    Some(value.checked_mul(num)? / den)
}
