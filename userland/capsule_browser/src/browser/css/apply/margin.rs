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

use alloc::vec::Vec;

use crate::browser::css::computed::Computed;
use crate::browser::css::set_len::set_len;
use crate::browser::css::sides::sides;

const MAX_MARGIN_PX: u32 = 128;

// Margin shorthand and sides, tracking auto on the horizontal axis so a
// centered block can be recognised downstream.
pub(super) fn apply_margin(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "margin" => {
            // Note which horizontal sides read `auto` before sides() folds
            // them to 0.
            let parts: Vec<&str> = value.split_whitespace().collect();
            let is_auto = |t: &str| t.eq_ignore_ascii_case("auto");
            let (r_auto, l_auto) = match parts.len() {
                1 => (is_auto(parts[0]), is_auto(parts[0])),
                2 | 3 => (is_auto(parts[1]), is_auto(parts[1])),
                4 => (is_auto(parts[1]), is_auto(parts[3])),
                _ => (false, false),
            };
            c.margin_right_auto = r_auto;
            c.margin_left_auto = l_auto;
            c.margin_auto_x = r_auto && l_auto;
            if let Some([t, r, b, l]) = sides(value, fs, MAX_MARGIN_PX) {
                c.margin_top = t;
                c.margin_right = r;
                c.margin_bottom = b;
                c.margin_left = l;
            }
        }
        "margin-top" => set_len(&mut c.margin_top, value, fs, MAX_MARGIN_PX),
        "margin-right" => {
            c.margin_right_auto = value.trim().eq_ignore_ascii_case("auto");
            c.margin_auto_x = c.margin_left_auto && c.margin_right_auto;
            set_len(&mut c.margin_right, value, fs, MAX_MARGIN_PX);
        }
        "margin-bottom" => set_len(&mut c.margin_bottom, value, fs, MAX_MARGIN_PX),
        "margin-left" => {
            c.margin_left_auto = value.trim().eq_ignore_ascii_case("auto");
            c.margin_auto_x = c.margin_left_auto && c.margin_right_auto;
            set_len(&mut c.margin_left, value, fs, MAX_MARGIN_PX);
        }
        _ => return false,
    }
    true
}
