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

use super::entries::ENTRIES;
use super::theme::{CYAN, CYAN_HOT, DIM};
use crate::display::font::{draw_string_2x, CHAR_WIDTH};
use crate::display::fx::{blend_rect, bloom_char};

const ROW_PITCH: u32 = 50;

pub(super) fn draw_list(w: u32, top: u32, sel: usize) {
    for (i, entry) in ENTRIES.iter().enumerate() {
        let label = entry.label().as_bytes();
        let lw = label.len() as u32 * CHAR_WIDTH * 2;
        let x = w.saturating_sub(lw) / 2;
        let y = top + i as u32 * ROW_PITCH;
        if i == sel {
            let ux = x.saturating_sub(8);
            blend_rect(ux, y + 33, lw + 16, 5, CYAN, 18);
            blend_rect(ux, y + 34, lw + 16, 3, CYAN, 60);
            let mut cx = x;
            for &ch in label {
                bloom_char(cx, y, ch, 2, CYAN_HOT, CYAN);
                cx += CHAR_WIDTH * 2;
            }
        } else {
            draw_string_2x(x, y, label, DIM);
        }
    }
}
