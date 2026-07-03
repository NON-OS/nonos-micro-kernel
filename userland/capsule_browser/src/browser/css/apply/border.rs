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

use crate::browser::css::border_parts::border_parts;
use crate::browser::css::color::parse_color;
use crate::browser::css::computed::Computed;
use crate::browser::css::set_len::set_len;
use crate::browser::css::sides::sides;

const MAX_BORDER_PX: u32 = 32;

// Border shorthands, per-side widths and color.
pub(super) fn apply_border(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "border" | "border-top" | "border-right" | "border-bottom" | "border-left" => {
            let (width, color) = border_parts(value, fs, MAX_BORDER_PX);
            if let Some(rgb) = color {
                c.border_color = rgb;
            }
            if let Some(px) = width {
                match name {
                    "border-top" => c.border_top = px,
                    "border-right" => c.border_right = px,
                    "border-bottom" => c.border_bottom = px,
                    "border-left" => c.border_left = px,
                    _ => {
                        c.border_top = px;
                        c.border_right = px;
                        c.border_bottom = px;
                        c.border_left = px;
                    }
                }
            }
        }
        "border-width" => {
            if let Some([t, r, b, l]) = sides(value, fs, MAX_BORDER_PX) {
                c.border_top = t;
                c.border_right = r;
                c.border_bottom = b;
                c.border_left = l;
            }
        }
        "border-top-width" => set_len(&mut c.border_top, value, fs, MAX_BORDER_PX),
        "border-right-width" => set_len(&mut c.border_right, value, fs, MAX_BORDER_PX),
        "border-bottom-width" => set_len(&mut c.border_bottom, value, fs, MAX_BORDER_PX),
        "border-left-width" => set_len(&mut c.border_left, value, fs, MAX_BORDER_PX),
        "border-color" => {
            if let Some(rgb) = parse_color(value) {
                c.border_color = rgb;
            }
        }
        _ => return false,
    }
    true
}
