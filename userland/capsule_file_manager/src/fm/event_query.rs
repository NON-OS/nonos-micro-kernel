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

use super::screen::Screen;
use super::screen_search::run_search;
use super::state::State;

// The store walk behind a search is not free, so the query is edited freely and
// only run on Enter rather than on every keystroke.
const QUERY_MAX: usize = 64;

/// Key handling while the Search surface is up. Escape clears a query and, once
/// there is nothing left to clear, leaves the surface. Backspace pops a whole
/// character, so a multi-byte one can never be cut in half.
pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.code {
        KEY_ESC => {
            if state.query.is_empty() {
                state.screen = Screen::Browse;
            } else {
                state.query.clear();
                state.hits.clear();
            }
        }
        KEY_BACKSPACE => {
            state.query.pop();
        }
        KEY_ENTER => {
            run_search(state);
            state.status =
                if state.hits.is_empty() { b"no matches" } else { b"click a result to open" };
        }
        code => {
            if let Some(ch) = char::from_u32(code) {
                if (ch.is_ascii_graphic() || ch == ' ') && state.query.len() < QUERY_MAX {
                    state.query.push(ch);
                }
            }
        }
    }
    EventOutcome::Repaint
}
