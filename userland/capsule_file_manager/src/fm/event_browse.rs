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

use nonos_app_skeleton::{
    EventOutcome, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_LEFT, KEY_RIGHT, KEY_UP,
};

use super::event_actions::run_action;
use super::event_modes::open_mode;
use super::event_move::move_cursor;
use super::event_open::open_selected;
use super::event_parent::open_parent;
use super::prompt_start::start_prompt;
use super::state::{PromptKind, State};

pub fn on_browse_key(state: &mut State, code: u32) -> EventOutcome {
    if let Some(outcome) = run_action(state, code).or_else(|| open_mode(state, code)) {
        return outcome;
    }
    match code {
        KEY_ESC => EventOutcome::Close,
        KEY_ENTER | KEY_RIGHT => open_selected(state),
        KEY_BACKSPACE | KEY_LEFT => open_parent(state),
        KEY_UP => move_cursor(state, -1),
        KEY_DOWN => move_cursor(state, 1),
        code if code == b'j' as u32 => move_cursor(state, 1),
        code if code == b'k' as u32 => move_cursor(state, -1),
        code if code == b'h' as u32 => open_parent(state),
        code if code == b'l' as u32 => open_selected(state),
        code if code == b'n' as u32 => start_prompt(state, PromptKind::NewFile, b"new file: "),
        code if code == b'm' as u32 => start_prompt(state, PromptKind::MkDir, b"mkdir: "),
        code if code == b'r' as u32 => start_prompt(state, PromptKind::Rename, b"rename to: "),
        code if code == b'd' as u32 => {
            start_prompt(state, PromptKind::Delete, b"delete? type y + Enter: ")
        }
        _ => EventOutcome::Idle,
    }
}
