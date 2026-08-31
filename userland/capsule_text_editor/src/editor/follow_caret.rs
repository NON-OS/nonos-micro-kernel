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

//! Scroll just enough to keep the caret's visual line on screen after an edit
//! or a move.

use super::mode::Mode;
use super::position_at::position_at;
use super::state::State;

pub(super) fn follow_caret(state: &mut State, rows: u32) {
    if state.mode == Mode::Document {
        return;
    }
    let (line, _) = position_at(&state.buf[..state.len], state.wrap_cols, state.caret);
    if line < state.scroll_line {
        state.scroll_line = line;
    } else if rows > 0 && line >= state.scroll_line + rows {
        state.scroll_line = line + 1 - rows;
    }
}
