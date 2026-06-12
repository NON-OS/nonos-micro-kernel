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

use super::constants::{CELL_WIDTH, TEXT_LEFT};
use super::draw_cursor::draw_cursor;
use crate::term::state::State;
use crate::term::theme::{FOREGROUND, PATH, PROMPT};

pub fn draw_input_line(state: &State, fb: &mut PaintBuffer, y: u32) {
    // Character cells that fit between the left inset and an equal right margin.
    let total_cells = (fb.width.saturating_sub(TEXT_LEFT * 2) / CELL_WIDTH) as usize;
    // Prompt is glyph + path + trailing space; cap the path to a third of the
    // line so a deep cwd never starves the area left to type in.
    let cwd = state.cwd.as_bytes();
    let take = cwd.len().min((total_cells / 3).max(1));
    let prompt_cells = 1 + take + 1;
    fb.text(TEXT_LEFT, y, b"\xd8", PROMPT);
    fb.text(TEXT_LEFT + CELL_WIDTH, y, &cwd[cwd.len() - take..], PATH);

    // Horizontal scroll: slide a body_cells-wide window so the cursor is always
    // on screen, showing the start of the line whenever it fits.
    let body = state.line.as_bytes();
    let cursor = state.line.cursor.min(body.len());
    let body_cells = total_cells.saturating_sub(prompt_cells).max(1);
    let scroll = if cursor < body_cells { 0 } else { cursor - body_cells + 1 };
    let end = (scroll + body_cells).min(body.len());
    let px = TEXT_LEFT + prompt_cells as u32 * CELL_WIDTH;
    fb.text(px, y, &body[scroll..end], FOREGROUND);
    let under = body.get(cursor).copied().unwrap_or(0);
    draw_cursor(fb, prompt_cells, cursor - scroll, y + 1, under);
}
