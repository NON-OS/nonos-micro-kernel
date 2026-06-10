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

use crate::display::constants::COLOR_ERROR;
use crate::display::font::draw_string;
use crate::display::gop::{fill_rect, get_dimensions, is_initialized};

pub fn show_error_screen(msg: &[u8]) {
    if !is_initialized() {
        return;
    }
    let (w, h) = get_dimensions();
    if w == 0 || h == 0 {
        return;
    }
    fill_rect(0, 0, w, h, 0xFF100000);
    let x = 40;
    let y = h / 2 - 40;
    draw_string(x, y, b"BOOT ERROR", COLOR_ERROR);
    draw_string(x, y + 24, msg, 0xFFFFFFFF);
    draw_string(x, y + 56, b"System will reset...", 0xFF888888);
}
