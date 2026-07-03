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

use super::computed::Size;
use super::parse_px::parse_px;

const MAX_PCT: f32 = 1000.0;

// Resolve a width/height value: auto, a length, or a percentage of the
// containing box.
pub(super) fn parse_size(value: &str, em_base: u32) -> Option<Size> {
    let v = value.trim();
    if v.eq_ignore_ascii_case("auto") {
        return Some(Size::Auto);
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
