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

pub fn on_printable(state: &mut State, byte: u8) -> EventOutcome {
    // While a search is running the keys go to the search, not the line. The
    // line is showing a match, and typing into it would edit a command the
    // reader has not chosen yet.
    if state.search.is_some() {
        crate::event::search_edit::search_type(state, byte);
        return EventOutcome::Repaint;
    }
    if !state.line.insert(byte) {
        return EventOutcome::Idle;
    }
    state.history.reset_cursor();
    state.scrollback.jump_bottom();
    EventOutcome::Repaint
}
