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

use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use super::state::{Mode, State};
use super::view::rebuild_view;

// Live incremental search: each keystroke edits the filter and rebuilds the
// view immediately. Escape clears it, Enter keeps it and returns to browsing.
pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.code {
        KEY_ESC => {
            state.filter.clear();
            state.mode = Mode::Browse;
            state.status = b"filter cleared";
            rebuild_view(state);
        }
        KEY_ENTER => {
            state.mode = Mode::Browse;
            state.status = b"click or Enter to open";
        }
        KEY_BACKSPACE => {
            state.filter.pop();
            rebuild_view(state);
        }
        code => {
            if let Some(ch) = char::from_u32(code) {
                if ch.is_ascii_graphic() && state.filter.len() < 48 {
                    state.filter.push(ch);
                    rebuild_view(state);
                }
            }
        }
    }
    EventOutcome::Repaint
}
