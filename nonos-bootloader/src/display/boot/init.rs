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

use super::logo::{LOGO, LOGO_LINE_HEIGHT};
use crate::display::constants::{COLOR_ACCENT, COLOR_TEXT_DIM, COLOR_TEXT_WHITE};
use crate::display::font::draw_string;
use crate::display::gop::{clear_screen, get_dimensions, is_initialized};

pub fn init_boot_screen() {
    if !is_initialized() {
        return;
    }
    clear_screen(0xFF000000);
    let (w, _) = get_dimensions();
    let x = (w.saturating_sub(240)) / 2;
    for (i, line) in LOGO.iter().enumerate() {
        let y = 24 + (i as u32) * LOGO_LINE_HEIGHT;
        let color = if i < 5 { COLOR_ACCENT } else { COLOR_TEXT_DIM };
        draw_string(x, y, line, color);
    }
    draw_string(40, 140, b"Boot Log:", COLOR_TEXT_WHITE);
    draw_string(40, 156, b"---------", COLOR_TEXT_DIM);
}

pub fn reset_animation() {}

pub fn tick_animation() {}
