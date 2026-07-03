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

use super::calc::eval_calc;
use super::computed::Size;
use super::parse_px::parse_px;

const MAX_PCT: f32 = 1000.0;
const CALC_LIMIT: f32 = 100_000.0;

// Resolve a width/height value: auto, a length, a percentage of the
// containing box, or a calc() expression carried to layout unresolved.
pub(super) fn parse_size(value: &str, em_base: u32) -> Option<Size> {
    let v = value.trim();
    if v.eq_ignore_ascii_case("auto") {
        return Some(Size::Auto);
    }
    if let Some(inner) = strip_calc(v) {
        let (px, pml) = eval_calc(inner, em_base)?;
        if !px.is_finite() || !pml.is_finite() || px.abs() > CALC_LIMIT || pml.abs() > CALC_LIMIT {
            return None;
        }
        if pml == 0.0 && px >= 0.0 {
            return Some(Size::Px((px + 0.5) as u32));
        }
        return Some(Size::Calc(px as i32, pml as i32));
    }
    if let Some(num) = v.strip_suffix('%') {
        let f = num.trim().parse::<f32>().ok()?;
        if f.is_finite() && (0.0..=MAX_PCT).contains(&f) {
            return Some(Size::Pct((f + 0.5) as u16));
        }
        return None;
    }
    parse_px(v, em_base).map(Size::Px)
}

// The body of a calc(...) wrapper, case-insensitive, None otherwise.
pub(super) fn strip_calc(v: &str) -> Option<&str> {
    if v.len() >= 6 && v[..5].eq_ignore_ascii_case("calc(") && v.ends_with(')') {
        return Some(&v[5..v.len() - 1]);
    }
    None
}
