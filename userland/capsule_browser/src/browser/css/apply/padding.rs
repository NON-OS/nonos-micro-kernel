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

use crate::browser::css::computed::Computed;
use crate::browser::css::set_len::set_len;
use crate::browser::css::sides::sides;

const MAX_PAD_PX: u32 = 128;

// Padding shorthand and per-side lengths.
pub(super) fn apply_padding(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "padding" => {
            if let Some([t, r, b, l]) = sides(value, fs, MAX_PAD_PX) {
                c.pad_top = t;
                c.pad_right = r;
                c.pad_bottom = b;
                c.pad_left = l;
            }
        }
        "padding-top" => set_len(&mut c.pad_top, value, fs, MAX_PAD_PX),
        "padding-right" => set_len(&mut c.pad_right, value, fs, MAX_PAD_PX),
        "padding-bottom" => set_len(&mut c.pad_bottom, value, fs, MAX_PAD_PX),
        "padding-left" => set_len(&mut c.pad_left, value, fs, MAX_PAD_PX),
        _ => return false,
    }
    true
}
