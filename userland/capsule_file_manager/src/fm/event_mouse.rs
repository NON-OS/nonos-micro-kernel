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

use nonos_app_skeleton::{InputEvent, InputKind};

use super::layout::{FIRST_ROW_Y, ROW_HEIGHT};
use super::state::{Mode, State};

pub fn select_row(state: &mut State, event: InputEvent) {
    if event.kind != InputKind::ButtonDown || !matches!(state.mode, Mode::Browse) {
        return;
    }
    let rel = event.y as i32 - FIRST_ROW_Y as i32 + 4;
    if rel < 0 {
        return;
    }
    let row = state.scroll + (rel as u32 / ROW_HEIGHT) as usize;
    if row < state.entries.len() {
        state.cursor = row;
    }
}
