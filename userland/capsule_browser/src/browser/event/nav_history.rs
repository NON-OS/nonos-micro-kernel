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

use crate::browser::state::State;

pub fn nav_history(state: &mut State, delta: i32) -> EventOutcome {
    if state.fetch.is_some() {
        return EventOutcome::Idle;
    }
    let next = state.hist_index + delta;
    if next < 0 || next >= state.history.len() as i32 {
        return EventOutcome::Idle;
    }
    state.hist_index = next;
    state.suppress_history_push = true;
    let url = state.history[next as usize].clone();
    state.address = url.clone();
    state.pending_nav = Some(url);
    EventOutcome::Repaint
}
