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

use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, KEY_ESC, MOD_CTRL};

use super::ctrl_open::ctrl_open;
use super::ctrl_save::ctrl_save;
use super::state::{PromptOp, State};

pub(super) fn start(state: &mut State, op: PromptOp) -> EventOutcome {
    state.prompt = Some(op);
    state.status = match op {
        PromptOp::Open => b"open path, Enter to load, Esc cancels",
        PromptOp::Save => b"save path, Enter to write, Esc cancels",
    };
    EventOutcome::Repaint
}

pub(super) fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    let Some(op) = state.prompt else { return EventOutcome::Idle };
    match event.code {
        KEY_ESC => {
            state.prompt = None;
            state.status = b"cancelled";
        }
        KEY_BACKSPACE => {
            if state.path_len > 0 {
                state.path_len -= 1;
            }
        }
        KEY_ENTER => {
            state.prompt = None;
            return match op {
                PromptOp::Open => ctrl_open(state),
                PromptOp::Save => ctrl_save(state),
            };
        }
        code => {
            if event.flags & MOD_CTRL == 0 {
                if let Some(ch) = char::from_u32(code) {
                    if ch.is_ascii_graphic() && state.path_len < 255 {
                        state.path[state.path_len] = ch as u8;
                        state.path_len += 1;
                    }
                }
            }
        }
    }
    EventOutcome::Repaint
}
