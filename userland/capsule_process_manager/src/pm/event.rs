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
    EventOutcome, InputEvent, InputKind, KEY_DOWN, KEY_END, KEY_ENTER, KEY_ESC, KEY_HOME,
    KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_UP,
};

use super::layout::{sort_at_x, HEADER_H};
use super::state::{Screen, Sort, State, SIGKILL, SIGTERM};

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    // S toggles panels from anywhere.
    if event.is_key_down() && matches!(event.code, 0x53 | 0x73) {
        state.toggle_security();
        return EventOutcome::Repaint;
    }
    match state.screen {
        Screen::Overview | Screen::Processes => process_event(state, event),
        Screen::Cpu | Screen::Memory | Screen::Authority | Screen::Security => {
            security_event(state, event)
        }
    }
}

fn process_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        // A click on the header row sorts by that column; below it selects.
        if event.y < HEADER_H as i32 {
            if let Some(sort) = sort_at_x(event.x) {
                state.set_sort(sort);
            }
        } else {
            state.select_at(event.y);
        }
        return EventOutcome::Repaint;
    }
    if event.kind == InputKind::Wheel {
        state.scroll_by(if event.delta_y > 0 { -3 } else { 3 });
        return EventOutcome::Repaint;
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    let big = state.visible.max(1) as i32;
    match event.code {
        KEY_ESC => return EventOutcome::Close,
        KEY_UP => state.move_selection(-1),
        KEY_DOWN => state.move_selection(1),
        KEY_PAGE_UP => state.move_selection(-big),
        KEY_PAGE_DOWN => state.move_selection(big),
        KEY_HOME => state.move_selection(-(state.rows.len() as i32)),
        KEY_END => state.move_selection(state.rows.len() as i32),
        // K ends, F force-kills the selection.
        0x4B | 0x6B => state.kill_selected(SIGTERM),
        0x46 | 0x66 => state.kill_selected(SIGKILL),
        // Sort by cpu, memory, name or pid.
        0x43 | 0x63 => state.set_sort(Sort::Cpu),
        0x4D | 0x6D => state.set_sort(Sort::Mem),
        0x4E | 0x6E => state.set_sort(Sort::Name),
        0x50 | 0x70 => state.set_sort(Sort::Pid),
        // R re-reads the table immediately.
        0x52 | 0x72 => state.refresh(),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

fn security_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        state.alert_select_at(event.y);
        return EventOutcome::Repaint;
    }
    if event.kind == InputKind::Wheel {
        state.alert_scroll_by(if event.delta_y > 0 { -3 } else { 3 });
        return EventOutcome::Repaint;
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    let big = state.alert_visible.max(1) as i32;
    match event.code {
        // Esc leaves the panel back to the process table.
        KEY_ESC => state.toggle_security(),
        KEY_UP => state.alert_move(-1),
        KEY_DOWN => state.alert_move(1),
        KEY_PAGE_UP => state.alert_move(-big),
        KEY_PAGE_DOWN => state.alert_move(big),
        KEY_HOME => state.alert_move(-(state.alerts.len() as i32)),
        KEY_END => state.alert_move(state.alerts.len() as i32),
        // Enter jumps to the process the finding names.
        KEY_ENTER => state.jump_to_alert(),
        0x52 | 0x72 => state.refresh(),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
