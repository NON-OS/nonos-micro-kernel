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

use crate::browser::css::computed::{Clear, Computed, Float};

// float takes a box to one side of its container and clear drops a box below
// the floats it names. A floated box also becomes block-level so it takes a
// definite width and height to flow content around.
pub(super) fn apply_float(c: &mut Computed, name: &str, value: &str) -> bool {
    match name {
        "float" => {
            c.float = match value.trim() {
                "left" => Float::Left,
                "right" => Float::Right,
                _ => Float::None,
            };
            if c.float != Float::None {
                c.is_block = true;
            }
            true
        }
        "clear" => {
            c.clear = match value.trim() {
                "left" => Clear::Left,
                "right" => Clear::Right,
                "both" => Clear::Both,
                _ => Clear::None,
            };
            true
        }
        _ => false,
    }
}
