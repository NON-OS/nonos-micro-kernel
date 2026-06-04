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
use crate::term::dimensions::COLS;
use crate::term::state::State;
use crate::term::theme::{FOREGROUND, PATH, PROMPT};

pub fn draw_input_line(state: &State, fb: &mut PaintBuffer, y: u32) {
    fb.text(TEXT_LEFT, y, b"\xd8", PROMPT);
    let cwd = state.cwd.as_bytes();
    let take = cwd.len().min(40);
    let start = cwd.len() - take;
    fb.text(TEXT_LEFT + CELL_WIDTH, y, &cwd[start..], PATH);
    let prompt_cells = 1 + take + 1;
    let px = TEXT_LEFT + prompt_cells as u32 * CELL_WIDTH;
    let body = state.line.as_bytes();
    let btake = body.len().min(COLS);
    fb.text(px, y, &body[..btake], FOREGROUND);
    draw_cursor(fb, prompt_cells, state.line.cursor, y + 1);
}
