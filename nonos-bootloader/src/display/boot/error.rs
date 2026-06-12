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

use super::layout::splash;
use crate::display::constants::{COLOR_BOX_BG, COLOR_ERROR, COLOR_TEXT_DIM, COLOR_TEXT_PRIMARY};
use crate::display::font::{draw_string, draw_string_2x};
use crate::display::gop::{draw_rect, fill_rect, get_dimensions, hline, is_initialized};

pub fn show_error_screen(msg: &[u8]) {
    if !is_initialized() {
        return;
    }
    let (w, h) = get_dimensions();
    if w == 0 || h == 0 {
        return;
    }
    fill_rect(0, 0, w, h, 0xFF0A0202);
    let lay = splash();
    let (x, pw) = (lay.col_x, lay.col_w);
    let y = h / 3;
    fill_rect(x, y, pw, 120, COLOR_BOX_BG);
    draw_rect(x, y, pw, 120, COLOR_ERROR);
    fill_rect(x + 12, y + 9, 6, 6, COLOR_ERROR);
    draw_string(x + 26, y + 4, b"verified boot halted", COLOR_TEXT_DIM);
    hline(x, y + 24, pw, COLOR_ERROR);
    draw_string_2x(x + 14, y + 36, b"BOOT ERROR", COLOR_ERROR);
    let max_chars = (pw.saturating_sub(28) / 8) as usize;
    draw_string(x + 14, y + 74, &msg[..msg.len().min(max_chars)], COLOR_TEXT_PRIMARY);
    let note = b"the system will reset; nothing unverified was run";
    draw_string(x + 14, y + 96, note, COLOR_TEXT_DIM);
}
