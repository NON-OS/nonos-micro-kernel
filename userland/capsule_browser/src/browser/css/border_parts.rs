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

use super::color::parse_color;
use super::parse_px::parse_px;

// "border: 1px solid #ccc" and friends: pick out the width and the color,
// skip the line-style token. Returns (width, color) with either side optional.
pub(super) fn border_parts(value: &str, em: u32, max: u32) -> (Option<u32>, Option<u32>) {
    let mut width = None;
    let mut color = None;
    for part in value.split_whitespace() {
        if width.is_none() {
            if let Some(px) = parse_px(part, em) {
                width = Some(px.min(max));
                continue;
            }
        }
        if color.is_none() {
            if let Some(rgb) = parse_color(part) {
                color = Some(rgb);
            }
        }
    }
    (width, color)
}
