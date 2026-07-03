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

use super::parse_px::parse_px;

// line-height accepts a length, a percentage of the font size, or a bare
// multiplier of the font size. 0 means "normal", derived at use.
pub(super) fn parse_line_height(value: &str, font_px: u32) -> Option<u32> {
    let v = value.trim();
    if v.eq_ignore_ascii_case("normal") {
        return Some(0);
    }
    if let Some(num) = v.strip_suffix('%') {
        let f = num.trim().parse::<f32>().ok()?;
        if f.is_finite() && (0.0..=1000.0).contains(&f) {
            return Some((f / 100.0 * font_px as f32 + 0.5) as u32);
        }
        return None;
    }
    if let Some(px) = parse_px(v, font_px) {
        return Some(px);
    }
    let f = v.parse::<f32>().ok()?;
    if f.is_finite() && (0.0..=100.0).contains(&f) {
        Some((f * font_px as f32 + 0.5) as u32)
    } else {
        None
    }
}
