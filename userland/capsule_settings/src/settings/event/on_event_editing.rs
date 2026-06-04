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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use crate::settings::state::{edit_cancel, State};

use super::commit_string::commit_string;
use super::push_text_char::push_text_char;

pub(super) fn on_event_editing(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => {
            edit_cancel(state);
            EventOutcome::Repaint
        }
        KEY_ENTER => {
            commit_string(state);
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => {
            state.edit.pop();
            EventOutcome::Repaint
        }
        c if push_text_char(state, c) => EventOutcome::Repaint,
        _ => EventOutcome::Idle,
    }
}
