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

use crate::browser::css::computed::{Computed, Position};
use crate::browser::css::parse_size::parse_size;

// Positioning scheme and the four inset offsets. fixed pins to a viewport we
// do not scroll separately; treating it as absolute keeps such elements
// visible and out of normal flow.
pub(super) fn apply_position(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "position" => match value.trim() {
            "static" => c.position = Position::Static,
            "relative" | "sticky" => c.position = Position::Relative,
            "absolute" | "fixed" => c.position = Position::Absolute,
            _ => {}
        },
        "top" => {
            if let Some(s) = parse_size(value, fs) {
                c.top = s;
            }
        }
        "right" => {
            if let Some(s) = parse_size(value, fs) {
                c.right = s;
            }
        }
        "bottom" => {
            if let Some(s) = parse_size(value, fs) {
                c.bottom = s;
            }
        }
        "left" => {
            if let Some(s) = parse_size(value, fs) {
                c.left = s;
            }
        }
        _ => return false,
    }
    true
}
