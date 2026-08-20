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
    EventOutcome, KEY_DOWN, KEY_END, KEY_ENTER, KEY_ESC, KEY_HOME, KEY_LEFT, KEY_PAGE_DOWN,
    KEY_PAGE_UP, KEY_RIGHT, KEY_TAB, KEY_UP,
};

use crate::settings::state::{cursor_down, cursor_end, cursor_home, cursor_page, cursor_up, State};

use super::adjust::adjust;
use super::next_section::{next_section, prev_section};
use super::toggle_or_inc::toggle_or_inc;

const KEY_SPACE: u32 = 0x20;

pub(super) fn on_event_browsing(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => EventOutcome::Close,
        KEY_TAB => repaint_after(state, next_section),
        KEY_UP => repaint_after(state, cursor_up),
        KEY_DOWN => repaint_after(state, cursor_down),
        KEY_HOME => repaint_after(state, cursor_home),
        KEY_END => repaint_after(state, cursor_end),
        KEY_PAGE_UP => repaint_page(state, false),
        KEY_PAGE_DOWN => repaint_page(state, true),
        KEY_LEFT => repaint_delta(state, -1),
        KEY_RIGHT => repaint_delta(state, 1),
        KEY_SPACE | KEY_ENTER => repaint_after(state, toggle_or_inc),
        c if c == b'[' as u32 => repaint_after(state, prev_section),
        c if c == b']' as u32 => repaint_after(state, next_section),
        _ => EventOutcome::Idle,
    }
}

fn repaint_after(state: &mut State, f: fn(&mut State)) -> EventOutcome {
    f(state);
    EventOutcome::Repaint
}

fn repaint_page(state: &mut State, down: bool) -> EventOutcome {
    cursor_page(state, down);
    EventOutcome::Repaint
}

fn repaint_delta(state: &mut State, delta: i32) -> EventOutcome {
    adjust(state, delta);
    EventOutcome::Repaint
}
