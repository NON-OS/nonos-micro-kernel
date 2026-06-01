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
    EventOutcome, InputEvent, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_LEFT, KEY_RIGHT,
    KEY_TAB, KEY_UP,
};

use crate::settings::state::{cursor_down, cursor_up, edit_cancel, State};

use super::adjust::adjust;
use super::commit_string::commit_string;
use super::next_category::next_category;
use super::prev_category::prev_category;
use super::push_text_char::push_text_char;
use super::toggle_or_inc::toggle_or_inc;

const KEY_SPACE: u32 = 0x20;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if state.editing {
        return on_event_editing(state, event.code);
    }
    on_event_browsing(state, event.code)
}

fn on_event_editing(state: &mut State, code: u32) -> EventOutcome {
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
        c => {
            if push_text_char(state, c) {
                EventOutcome::Repaint
            } else {
                EventOutcome::Idle
            }
        }
    }
}

fn on_event_browsing(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => EventOutcome::Close,
        KEY_TAB => {
            next_category(state);
            EventOutcome::Repaint
        }
        KEY_UP => {
            cursor_up(state);
            EventOutcome::Repaint
        }
        KEY_DOWN => {
            cursor_down(state);
            EventOutcome::Repaint
        }
        KEY_LEFT => {
            adjust(state, -1);
            EventOutcome::Repaint
        }
        KEY_RIGHT => {
            adjust(state, 1);
            EventOutcome::Repaint
        }
        KEY_SPACE | KEY_ENTER => {
            toggle_or_inc(state);
            EventOutcome::Repaint
        }
        c if c == b'[' as u32 => {
            prev_category(state);
            EventOutcome::Repaint
        }
        c if c == b']' as u32 => {
            next_category(state);
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}
