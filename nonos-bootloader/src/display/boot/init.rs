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

use super::layout::splash;
use crate::display::font::{draw_char, CHAR_WIDTH};
use crate::display::fx::fill_atmosphere;
use crate::display::gop::{get_dimensions, is_initialized};

const SUBTITLE: &[u8] = b"VERIFIED BOOT";
const SUB_TRACK: u32 = 6;
const SUB_COLOR: u32 = 0xFF46AEB6;

pub fn init_boot_screen() {
    if !is_initialized() {
        return;
    }
    fill_atmosphere();
    let lay = splash();
    super::wordmark::draw_wordmark(0, lay.wordmark_y);

    let (w, _) = get_dimensions();
    let adv = CHAR_WIDTH + SUB_TRACK;
    let sw = SUBTITLE.len() as u32 * adv - SUB_TRACK;
    let mut sx = w.saturating_sub(sw) / 2;
    for &ch in SUBTITLE {
        draw_char(sx, lay.subtitle_y, ch, SUB_COLOR);
        sx += adv;
    }

    crate::display::log_panel::redraw_all();
}

pub fn reset_animation() {}

pub fn tick_animation() {}
