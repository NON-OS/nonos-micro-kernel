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

use nonos_app_skeleton::EventOutcome;

use super::scroll::ensure_visible;
use super::state::State;

pub fn move_cursor(state: &mut State, delta: i32) -> EventOutcome {
    if delta < 0 {
        state.cursor = state.cursor.saturating_sub((-delta) as usize);
    } else {
        state.cursor = state.cursor.saturating_add(delta as usize);
        state.cursor = state.cursor.min(state.entries.len().saturating_sub(1));
    }
    ensure_visible(state);
    EventOutcome::Repaint
}
