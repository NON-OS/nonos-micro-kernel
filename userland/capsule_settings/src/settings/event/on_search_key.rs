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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_UP};

use crate::settings::state::{search_clear, set_section, track_scroll, State};
use crate::settings::ui::bytes::as_str;
use crate::settings::ui::results;

/// Widest query the titlebar field can show without the text running under its
/// rounded right edge, so typing stops rather than scrolling out of sight.
const QUERY_MAX: usize = 24;

pub(super) fn on_search_key(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => {
            search_clear(state);
            state.search_focused = false;
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => {
            state.search.pop();
            state.search_cursor = 0;
            state.search_scroll = 0;
            EventOutcome::Repaint
        }
        KEY_UP => step(state, -1),
        KEY_DOWN => step(state, 1),
        KEY_ENTER => open_selected(state),
        c @ 0x20..=0x7E if state.search.len < QUERY_MAX => {
            state.search.push(c as u8);
            state.search_cursor = 0;
            state.search_scroll = 0;
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}

fn step(state: &mut State, delta: i32) -> EventOutcome {
    let n = results::count(as_str(state.search.as_slice()));
    if n == 0 {
        return EventOutcome::Idle;
    }
    let next = (state.search_cursor as i32 + delta).clamp(0, n as i32 - 1);
    state.search_cursor = next as usize;
    EventOutcome::Repaint
}

pub(super) fn open_selected(state: &mut State) -> EventOutcome {
    let query = as_str(state.search.as_slice());
    let Some((section, index, _)) = results::at(query, state.search_cursor) else {
        return EventOutcome::Idle;
    };
    search_clear(state);
    state.search_focused = false;
    set_section(state, section);
    state.cursor[section.index()] = index;
    track_scroll(state);
    EventOutcome::Repaint
}
