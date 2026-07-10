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

//! Place the caret where the pointer clicked in the text body.

use super::byte_at::byte_at;
use super::layout::{text_left, LINE_HEIGHT, PAD_TOP};
use super::state::State;

pub(super) fn click_caret(state: &mut State, x: i32, y: i32) {
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (x as u32, y as u32);
    let top = state.pane_y + PAD_TOP;
    if y < top {
        state.caret = 0;
        return;
    }
    let row = (y - top) / LINE_HEIGHT;
    let line = state.scroll_line + row;
    // Same measured cell width and pane origin the body was drawn with, so
    // clicks land on the character under the pointer.
    let col = x.saturating_sub(text_left(state.pane_x)) / state.glyph_advance.max(1);
    state.caret = byte_at(&state.buf[..state.len], state.wrap_cols, line, col);
}
