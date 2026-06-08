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

use crate::display::font::draw_string;
use crate::display::gop::{fill_rect, get_dimensions};
use crate::menu::brand;
use super::layout::{get_panel_bounds, MARGIN, PAD};

pub fn draw_panel_background(x: u32, y: u32, w: u32) {
    let (_, sh) = get_dimensions();
    let h = sh - y - MARGIN - 60;
    fill_rect(x, y, w, h, brand::BG_CARD);
    fill_rect(x, y, w, 3, brand::ACCENT_PRIMARY);
    fill_rect(x, y, 1, h, brand::BORDER);
    fill_rect(x + w - 1, y, 1, h, brand::BORDER);
    fill_rect(x, y + h - 1, w, 1, brand::BORDER);
}

pub fn draw_panel_header(x: u32, y: u32, w: u32) {
    draw_string(x + PAD, y + 20, b"Boot Options", brand::ACCENT_PRIMARY);
    fill_rect(x + PAD, y + 50, w - PAD * 2, 1, brand::BORDER);
}

pub fn clear_menu_area() { let (px, py, pw, ph) = get_panel_bounds(); fill_rect(px, py, pw, ph, brand::BG_PRIMARY); }
pub fn clear_screen() { crate::display::draw_vignette(); }
