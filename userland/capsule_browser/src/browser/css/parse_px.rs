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

use crate::browser::manifest::{HEIGHT, WIDTH};

use super::strip_unit::strip_unit;

const ROOT_EM: f32 = 16.0;

// Resolve a CSS length to whole pixels. Supports px, em (against em_base),
// rem (against the 16px root) and vw/vh (against the viewport). Anything
// else is rejected.
pub(super) fn parse_px(value: &str, em_base: u32) -> Option<u32> {
    let v = value.trim();
    // calc() with no percentage part is a plain length here.
    if let Some(inner) = super::parse_size::strip_calc(v) {
        let (px, pml) = super::calc::eval_calc(inner, em_base)?;
        if pml == 0.0 && px.is_finite() && (0.0..=100_000.0).contains(&px) {
            return Some((px + 0.5) as u32);
        }
        return None;
    }
    if v == "0" {
        return Some(0);
    }
    // rem before em: "rem" also ends in "em".
    let (num, unit_px) = if let Some(n) = strip_unit(v, "px") {
        (n, 1.0)
    } else if let Some(n) = strip_unit(v, "rem") {
        (n, ROOT_EM)
    } else if let Some(n) = strip_unit(v, "em") {
        (n, em_base as f32)
    } else if let Some(n) = strip_unit(v, "vw") {
        (n, WIDTH as f32 / 100.0)
    } else if let Some(n) = strip_unit(v, "vh") {
        (n, HEIGHT as f32 / 100.0)
    } else {
        return None;
    };
    let f = num.trim().parse::<f32>().ok()?;
    let px = f * unit_px;
    // Cap the length like the calc() branch above. Without a ceiling a value
    // such as width:4000000000px reaches layout as ~4.29e9 and overflows the
    // i32 box-model arithmetic, which aborts under release overflow-checks.
    if px.is_finite() && (0.0..=100_000.0).contains(&px) {
        Some((px + 0.5) as u32)
    } else {
        None
    }
}
