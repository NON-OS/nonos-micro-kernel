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

//! Ctrl combined with a navigation key: word motion (Ctrl+Left/Right), word
//! deletion (Ctrl+Backspace), and jump to file start/end (Ctrl+Home/End).
//! Holding Shift extends the selection, matching the plain arrow keys.

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_END, KEY_HOME, KEY_LEFT, KEY_RIGHT};

use super::follow_caret::follow_caret;
use super::state::State;

pub(super) fn on_ctrl_nav(
    state: &mut State,
    code: u32,
    rows: u32,
    shift: bool,
) -> Option<EventOutcome> {
    let motion = matches!(code, KEY_LEFT | KEY_RIGHT | KEY_HOME | KEY_END);
    if motion {
        if shift {
            state.begin_sel();
        } else {
            state.clear_sel();
        }
        match code {
            KEY_LEFT => state.word_left(),
            KEY_RIGHT => state.word_right(),
            KEY_HOME => state.caret = 0,
            KEY_END => state.caret = state.len,
            _ => {}
        }
        follow_caret(state, rows);
        return Some(EventOutcome::Repaint);
    }
    if code == KEY_BACKSPACE {
        let changed = state.delete_sel() || state.delete_word_left();
        if changed {
            state.status = b"edited";
            follow_caret(state, rows);
            return Some(EventOutcome::Repaint);
        }
        return Some(EventOutcome::Idle);
    }
    None
}
