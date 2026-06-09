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

use nonos_toolkit::decorations::{draw_maximize_button, maximize_button_rect};

const MAX_FILL_ARGB: u32 = 0xFF3F_B950;
const MAX_GLYPH_ARGB: u32 = 0xFF10_2010;

pub(super) fn paint_maximize_button(pixels: &mut [u32], stride_words: usize, width: u32) {
    let rect = maximize_button_rect(width);
    draw_maximize_button(pixels, stride_words, width, &rect, MAX_FILL_ARGB, MAX_GLYPH_ARGB);
}
