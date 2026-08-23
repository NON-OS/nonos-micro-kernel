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

use nonos_app_skeleton::{EventOutcome, KEY_ESC, KEY_TAB};

use super::super::state::{Screen, Sort, State, SCREENS, SIGKILL, SIGTERM};
use super::event_scroll::scroll_key;

pub fn key(state: &mut State, code: u32) -> EventOutcome {
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
        0x53 | 0x73 => state.set_screen(Screen::Security),
        0x4B | 0x6B => state.kill_selected(SIGTERM),
        0x46 | 0x66 => state.kill_selected(SIGKILL),
        0x43 | 0x63 => state.set_sort(Sort::Cpu),
        0x4D | 0x6D => state.set_sort(Sort::Mem),
        0x4E | 0x6E => state.set_sort(Sort::Name),
        0x50 | 0x70 => state.set_sort(Sort::Pid),
        0x52 | 0x72 => state.refresh(),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

// The digit row selects a screen by the position its nav row is drawn at.
fn ordinal(code: u32) -> Option<Screen> {
    (0x31..=0x36).contains(&code).then(|| SCREENS[(code - 0x31) as usize])
}
