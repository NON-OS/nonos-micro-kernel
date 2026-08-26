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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ESC, KEY_TAB};

use super::super::state::{Filter, Screen, Sort, State, SCREENS, SIGKILL, SIGTERM};
use super::event_scroll::scroll_key;

pub fn key(state: &mut State, code: u32) -> EventOutcome {
    if state.query_focused() {
        if let Some(outcome) = typing(state, code) {
            return outcome;
        }
    }
    if let Some(screen) = ordinal(code) {
        state.set_screen(screen);
        return EventOutcome::Repaint;
    }
    if scroll_key(state, code) {
        return EventOutcome::Repaint;
    }
    match code {
        KEY_ESC => return EventOutcome::Close,
        KEY_TAB => state.set_screen(state.screen.next()),
        0x53 | 0x73 => state.toggle_security(),
        0x4B | 0x6B => state.kill_selected(SIGTERM),
        0x46 | 0x66 => state.kill_selected(SIGKILL),
        0x43 | 0x63 => state.set_sort(Sort::Cpu),
        0x4D | 0x6D => state.set_sort(Sort::Mem),
        0x4E | 0x6E => state.set_sort(Sort::Name),
        0x50 | 0x70 => state.set_sort(Sort::Pid),
        0x52 | 0x72 => state.refresh(),
        0x41 | 0x61 => state.set_filter(Filter::All),
        0x45 | 0x65 => state.set_filter(Filter::Elevated),
        0x54 | 0x74 => state.set_filter(Filter::Protected),
        0x47 | 0x67 => state.set_filter(Filter::Flagged),
        0x2F => state.focus_search(true),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

// While the field owns the keyboard, printable ASCII edits the query instead of
// firing a letter shortcut, and Escape empties the field rather than the window.
// Every navigation code is >= 0x1201 and so falls through untouched, which is
// what lets the arrows keep driving the selection while the user types.
fn typing(state: &mut State, code: u32) -> Option<EventOutcome> {
    match code {
        KEY_ESC => {
            state.clear_query();
            state.focus_search(false);
        }
        KEY_BACKSPACE => state.pop_char(),
        0x20..=0x7E => state.push_char(code as u8),
        _ => return None,
    }
    Some(EventOutcome::Repaint)
}

// The digit row selects a screen by the position its nav row is drawn at.
fn ordinal(code: u32) -> Option<Screen> {
    (0x31..=0x36).contains(&code).then(|| SCREENS[(code - 0x31) as usize])
}
