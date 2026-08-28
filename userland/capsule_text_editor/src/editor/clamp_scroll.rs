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

use super::max_scroll::max_scroll;
use super::mode::Mode;
use super::state::State;
use super::visual_lines::visual_lines;

pub(super) fn clamp_scroll(state: &mut State, rows: u32) {
    if state.mode == Mode::Document {
        return;
    }
    let total = visual_lines(&state.buf[..state.len], state.wrap_cols);
    state.scroll_line = state.scroll_line.min(max_scroll(total, rows));
}
