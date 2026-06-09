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

use nonos_toolkit::decorations::{draw_minimize_button, minimize_button_rect};

const MIN_FILL_ARGB: u32 = 0xFF4B_5563;
const MIN_GLYPH_ARGB: u32 = 0xFFE6_EDF3;

pub(super) fn paint_minimize_button(pixels: &mut [u32], stride_words: usize, width: u32) {
    let rect = minimize_button_rect(width);
    draw_minimize_button(pixels, stride_words, width, &rect, MIN_FILL_ARGB, MIN_GLYPH_ARGB);
}
