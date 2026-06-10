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

use nonos_toolkit::decorations::{close_button_rect, draw_close_button};

const CLOSE_FILL_ARGB: u32 = 0xFFD0_4B4B;
const CLOSE_HOVER_ARGB: u32 = 0xFFE8_7070;
const CLOSE_GLYPH_ARGB: u32 = 0xFFFF_FFFF;

pub(super) fn paint_close_button(
    pixels: &mut [u32],
    stride_words: usize,
    width: u32,
    hovered: bool,
) {
    let rect = close_button_rect(width);
    let fill = if hovered { CLOSE_HOVER_ARGB } else { CLOSE_FILL_ARGB };
    draw_close_button(pixels, stride_words, width, &rect, fill, CLOSE_GLYPH_ARGB);
}
