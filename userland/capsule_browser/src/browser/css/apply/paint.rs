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

use crate::browser::css::color::parse_color;
use crate::browser::css::computed::Computed;
use crate::browser::css::parse_px::parse_px;

// Painted appearance: background color, corner radius, stacking level and
// overflow clipping.
pub(super) fn apply_paint(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "background" | "background-color" => {
            // The shorthand may carry url()/repeat tokens; take the color.
            for part in value.split_whitespace() {
                if let Some(rgb) = parse_color(part) {
                    c.bg = rgb;
                    break;
                }
            }
        }
        // One radius for all corners; the first value of a corner list wins.
        "border-radius" => {
            if let Some(first) = value.split_whitespace().next() {
                if let Some(px) = parse_px(first, fs) {
                    c.radius = px.min(64);
                }
            }
        }
        "overflow" | "overflow-x" | "overflow-y" => match value.trim() {
            "hidden" | "clip" => c.overflow_hidden = true,
            "visible" | "auto" | "scroll" => c.overflow_hidden = false,
            _ => {}
        },
        "z-index" => {
            if let Ok(z) = value.trim().parse::<i32>() {
                c.z = z.clamp(-999, 999);
            }
        }
        _ => return false,
    }
    true
}
