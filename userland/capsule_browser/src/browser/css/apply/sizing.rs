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

use crate::browser::css::computed::{Computed, ObjectFit, Size};
use crate::browser::css::parse_size::parse_size;

// Width and height with their min/max clamps. `none` lifts a clamp.
pub(super) fn apply_sizing(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "box-sizing" => match value.trim() {
            "border-box" => c.border_box = true,
            "content-box" => c.border_box = false,
            _ => {}
        },
        "object-fit" => match value.trim() {
            "cover" => c.object_fit = ObjectFit::Cover,
            "fill" => c.object_fit = ObjectFit::Fill,
            "contain" | "scale-down" | "none" => c.object_fit = ObjectFit::Contain,
            _ => {}
        },
        "width" => {
            if let Some(s) = parse_size(value, fs) {
                c.width = s;
            }
        }
        "max-width" => {
            if value.trim().eq_ignore_ascii_case("none") {
                c.max_width = Size::Auto;
            } else if let Some(s) = parse_size(value, fs) {
                c.max_width = s;
            }
        }
        "min-width" => {
            if let Some(s) = parse_size(value, fs) {
                c.min_width = s;
            }
        }
        "height" => {
            if let Some(s) = parse_size(value, fs) {
                c.height = s;
            }
        }
        "min-height" => {
            if value.trim().eq_ignore_ascii_case("none") {
                c.min_height = Size::Auto;
            } else if let Some(s) = parse_size(value, fs) {
                c.min_height = s;
            }
        }
        "max-height" => {
            if value.trim().eq_ignore_ascii_case("none") {
                c.max_height = Size::Auto;
            } else if let Some(s) = parse_size(value, fs) {
                c.max_height = s;
            }
        }
        _ => return false,
    }
    true
}
