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

use crate::display::constants::{
    COLOR_BORDER, COLOR_BOX_BG, COLOR_SUCCESS, COLOR_TEXT_DIM, COLOR_TEXT_MUTED,
    COLOR_TEXT_PRIMARY, COLOR_WARNING,
};
use crate::display::font::draw_string;
use crate::display::gop::{draw_rect, fill_rect};

pub fn chip_width(label: &[u8]) -> u32 {
    20 + label.len() as u32 * 8 + 10
}

pub fn draw_chip(x: u32, y: u32, label: &[u8], state: u8) -> u32 {
    let (dot, text) = match state {
        2 => (COLOR_SUCCESS, COLOR_TEXT_PRIMARY),
        1 => (COLOR_WARNING, COLOR_TEXT_PRIMARY),
        _ => (COLOR_TEXT_MUTED, COLOR_TEXT_DIM),
    };
    let w = chip_width(label);
    fill_rect(x, y, w, 20, COLOR_BOX_BG);
    draw_rect(x, y, w, 20, COLOR_BORDER);
    fill_rect(x + 8, y + 7, 6, 6, dot);
    draw_string(x + 20, y + 2, label, text);
    x + w + 10
}
