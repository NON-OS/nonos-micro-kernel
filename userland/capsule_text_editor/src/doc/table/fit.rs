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

//! Trim a cell to the width its column actually got. Glyphs are proportional,
//! so the cut is found by measuring shorter and shorter prefixes rather than by
//! counting characters, and it only ever lands on a UTF-8 boundary.

use crate::doc::measure::Measurer;
use crate::doc::style::RunStyle;

pub fn fit<'a>(text: &'a str, max_w: f32, style: &RunStyle, m: &dyn Measurer) -> &'a str {
    if max_w <= 0.0 {
        return "";
    }
    if m.advance(text, style) <= max_w {
        return text;
    }
    let mut end = text.len();
    while end > 0 {
        end -= 1;
        while end > 0 && !text.is_char_boundary(end) {
            end -= 1;
        }
        if m.advance(&text[..end], style) <= max_w {
            break;
        }
    }
    &text[..end]
}
