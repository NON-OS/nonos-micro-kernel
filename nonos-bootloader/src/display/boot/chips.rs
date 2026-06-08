// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::constants::{COLOR_BORDER, COLOR_SUCCESS, COLOR_TEXT_DIM, COLOR_WARNING};
use crate::display::font::draw_string;
use crate::display::gop::draw_rect;

pub fn draw_chip(x: u32, y: u32, label: &[u8], state: u8) -> u32 {
    let color = match state {
        2 => COLOR_SUCCESS,
        1 => COLOR_WARNING,
        _ => COLOR_TEXT_DIM,
    };
    let w = label.len() as u32 * 8 + 12;
    draw_rect(x, y, w, 18, COLOR_BORDER);
    draw_string(x + 6, y + 1, label, color);
    x + w + 8
}
