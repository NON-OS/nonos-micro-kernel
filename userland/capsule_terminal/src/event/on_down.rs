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

use crate::term::state::State;

pub fn on_down(state: &mut State) -> EventOutcome {
    match state.history.next_matching(&state.hist_prefix) {
        // Ran past the newest match: restore the text the search began from.
        Some([]) => {
            state.line.replace(&state.hist_prefix);
            EventOutcome::Repaint
        }
        Some(entry) => {
            state.line.replace(entry);
            EventOutcome::Repaint
        }
        None => EventOutcome::Idle,
    }
}
