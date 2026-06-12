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

use crate::display::constants::{COLOR_ACCENT, COLOR_BORDER, COLOR_BOX_BG, COLOR_TEXT_MUTED};
use crate::display::font::draw_string;
use crate::display::gop::{draw_rect, fill_rect, hline};

pub fn draw_panel(x: u32, y: u32, w: u32, h: u32, title: &[u8]) {
    fill_rect(x, y, w, h, COLOR_BOX_BG);
    draw_rect(x, y, w, h, COLOR_BORDER);
    fill_rect(x + 12, y + 7, 6, 6, COLOR_ACCENT);
    draw_string(x + 26, y + 2, title, COLOR_TEXT_MUTED);
    hline(x, y + 22, w, COLOR_BORDER);
}

pub fn panel_line(x: u32, y: u32, prefix: &[u8], prefix_color: u32, text: &[u8], text_color: u32) {
    draw_string(x + 8, y, prefix, prefix_color);
    draw_string(x + 40, y, text, text_color);
}
