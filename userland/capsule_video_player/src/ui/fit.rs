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

use nonos_app_skeleton::paint::measure_ttf;

pub fn width(s: &str, px: f32) -> u32 {
    measure_ttf(s, px).max(0) as u32
}

pub fn fit(s: &str, max: u32, px: f32) -> &str {
    if width(s, px) <= max {
        return s;
    }
    let mut end = s.len();
    while end > 0 {
        end -= 1;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        if width(&s[..end], px) <= max {
            break;
        }
    }
    &s[..end]
}

pub fn center_x(s: &str, x: u32, w: u32, px: f32) -> i32 {
    let text = width(s, px);
    (x + w.saturating_sub(text) / 2) as i32
}

pub fn right_x(s: &str, right: u32, px: f32) -> i32 {
    right.saturating_sub(width(s, px)) as i32
}
