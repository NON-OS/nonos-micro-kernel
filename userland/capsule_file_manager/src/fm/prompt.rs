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

use alloc::string::String;

use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use super::prompt_commit::commit;
use super::state::{Mode, State};

pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    let Mode::Prompt(kind) = state.mode else { return EventOutcome::Idle };
    match event.code {
        KEY_ESC => {
            state.mode = Mode::Browse;
            state.input = String::new();
            state.status = b"cancelled";
        }
        KEY_BACKSPACE => {
            state.input.pop();
        }
        KEY_ENTER => commit(state, kind),
        code => {
            if let Some(ch) = char::from_u32(code) {
                if ch.is_ascii_graphic() && state.input.len() < 64 {
                    state.input.push(ch);
                }
            }
        }
    }
    EventOutcome::Repaint
}
