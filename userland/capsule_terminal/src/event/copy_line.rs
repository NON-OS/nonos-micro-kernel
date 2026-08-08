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

use nonos_app_skeleton::{clipboard_copy, EventOutcome};

use crate::term::state::State;

pub fn copy_line(state: &mut State) -> EventOutcome {
    // A failed copy said nothing, so the user pasted stale clipboard contents
    // somewhere else and had no way to know why.
    if clipboard_copy(state.line.as_bytes()).is_err() {
        state.scrollback.push_line(b"copy: clipboard unavailable");
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}
