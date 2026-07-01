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

use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, MOD_SHIFT};

use crate::browser::keymap::printable;
use crate::browser::state::State;

pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.code {
        KEY_ENTER => {
            if crate::browser::proxy::command(state, &state.address.clone()) {
                state.address_focused = false;
                return EventOutcome::Repaint;
            }
            state.pending_nav = Some(state.address.clone());
            state.status = alloc::format!("loading {}", state.address);
            state.address_focused = false;
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => {
            state.address.pop();
            EventOutcome::Repaint
        }
        code => match printable(code, event.flags & MOD_SHIFT != 0) {
            Some(b) => {
                state.address.push(b as char);
                EventOutcome::Repaint
            }
            None => EventOutcome::Idle,
        },
    }
}
