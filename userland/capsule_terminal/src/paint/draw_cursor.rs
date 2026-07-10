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

use nonos_app_skeleton::PaintBuffer;

use super::constants::{CELL_WIDTH, LINE_HEIGHT, TEXT_LEFT};
use crate::term::theme::{BACKGROUND, CURSOR};

pub fn draw_cursor(
    fb: &mut PaintBuffer,
    prompt_cells: usize,
    cursor_cell: usize,
    baseline_y: u32,
    under: u8,
    scale: u32,
) {
    let cw = CELL_WIDTH * scale;
    let lh = LINE_HEIGHT * scale;
    let x = TEXT_LEFT + (prompt_cells as u32 + cursor_cell as u32) * cw;
    fb.fill_rect(x, baseline_y, cw, lh.saturating_sub(2), CURSOR);
    // Inverse block: when the cursor sits on a printable glyph, repaint it in
    // the background colour so the character reads through the block.
    if under > b' ' {
        fb.text_scaled(x, baseline_y.saturating_sub(1), &[under], BACKGROUND, scale);
    }
}
