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
use super::panel::draw_panel;
use crate::display::constants::COLOR_TEXT_DIM;
use crate::display::font::draw_string;
use crate::display::gop::{get_dimensions, is_initialized};

const SUBTITLE: &[u8] = b"secure attestation boot";

pub fn init_boot_screen() {
    if !is_initialized() {
        return;
    }
    super::vignette::draw_vignette();
    let lay = splash();
    super::wordmark::draw_wordmark(0, lay.wordmark_y);
    let (w, _) = get_dimensions();
    let sx = (w.saturating_sub(SUBTITLE.len() as u32 * 8)) / 2;
    draw_string(sx, lay.subtitle_y, SUBTITLE, COLOR_TEXT_DIM);
    draw_panel(lay.panel_x, lay.panel_y, lay.panel_w, lay.panel_h, b"verified boot");
    crate::display::log_panel::redraw_all();
}

pub fn reset_animation() {}

pub fn tick_animation() {}
