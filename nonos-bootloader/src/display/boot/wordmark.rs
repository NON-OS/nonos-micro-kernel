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

use crate::display::font::CHAR_WIDTH;
use crate::display::fx::bloom_char;
use crate::display::gop::{get_dimensions, is_initialized};

const WORD: [u8; 5] = [b'N', 0xD8, b'N', b'O', b'S'];
const TRACK: u32 = 10;
const TITLE: u32 = 0xFFEAFDFA;
const GLOW: u32 = 0xFF00F5D4;

fn advance() -> u32 {
    CHAR_WIDTH * 3 + TRACK
}

pub fn wordmark_width() -> u32 {
    WORD.len() as u32 * advance() - TRACK
}

pub fn draw_wordmark(_x: u32, y: u32) {
    if !is_initialized() {
        return;
    }
    let (w, _) = get_dimensions();
    let mut x = w.saturating_sub(wordmark_width()) / 2;
    for &ch in WORD.iter() {
        bloom_char(x, y, ch, 3, TITLE, GLOW);
        x += advance();
    }
}
