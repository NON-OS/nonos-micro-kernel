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
    // Save As starts from the current path so a small edit is quick. Open
    // starts empty, because opening a different file meant backspacing the
    // whole of the old one first.
    match op {
        PromptOp::Open => state.prompt_len = 0,
        PromptOp::Save => {
            state.prompt_path[..state.path_len].copy_from_slice(&state.path[..state.path_len]);
            state.prompt_len = state.path_len;
        }
    }
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
            // The document keeps the path it had; only the scratch is dropped.
            state.prompt = None;
            state.prompt_len = 0;
            state.status = b"cancelled";
        }
        KEY_BACKSPACE => {
            if state.prompt_len > 0 {
                state.prompt_len -= 1;
            }
        }
        KEY_ENTER => {
            state.prompt = None;
            if state.prompt_len == 0 {
                state.status = b"no path given";
                return EventOutcome::Repaint;
            }
            // Commit point: the typed path becomes the document's only here.
            state.path[..state.prompt_len].copy_from_slice(&state.prompt_path[..state.prompt_len]);
            state.path_len = state.prompt_len;
            state.prompt_len = 0;
            return match op {
                PromptOp::Open => ctrl_open(state),
                PromptOp::Save => ctrl_save(state),
            };
        }
        code => {
            if event.flags & MOD_CTRL == 0 {
                if let Some(ch) = char::from_u32(code) {
                    if ch.is_ascii_graphic() && state.prompt_len < 255 {
                        state.prompt_path[state.prompt_len] = ch as u8;
                        state.prompt_len += 1;
                    }
                }
            }
        }
    }
    EventOutcome::Repaint
}
