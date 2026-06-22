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

use super::theme::{CYAN, TITLE};
use crate::display::font::CHAR_WIDTH;
use crate::display::fx::bloom_char;

// 0xD8 is the slashed-O glyph: the title reads N O N O S.
const TITLE_BYTES: [u8; 5] = [b'N', 0xD8, b'N', b'O', b'S'];
const TITLE_TRACK: u32 = 10;

pub(super) fn draw_header(w: u32, top: u32) {
    let adv = CHAR_WIDTH * 3 + TITLE_TRACK;
    let tw = TITLE_BYTES.len() as u32 * adv - TITLE_TRACK;
    let mut x = w.saturating_sub(tw) / 2;
    for &ch in TITLE_BYTES.iter() {
        bloom_char(x, top, ch, 3, TITLE, CYAN);
        x += adv;
    }
}
