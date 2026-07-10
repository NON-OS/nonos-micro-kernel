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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, KEY_ESC};

use crate::wallet::state::State;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.kind {
        InputKind::KeyDown if event.code == KEY_ESC => EventOutcome::Close,
        InputKind::KeyDown => {
            record(state, 0, event.x, event.y);
            force_repaint(super::on_key::on_key(state, event.code))
        }
        InputKind::ButtonDown => {
            record(state, 5, event.x, event.y);
            force_repaint(super::on_pointer::on_pointer(state, event.x, event.y))
        }
        _ => EventOutcome::Idle,
    }
}

// Record a discrete input event so the status bar can show, live, whether
// key and pointer events are actually reaching the capsule. Motion events are
// deliberately excluded so the readout is not spammed and stays cheap.
fn record(state: &mut State, kind: u32, x: i32, y: i32) {
    state.in_count = state.in_count.wrapping_add(1);
    state.in_kind = kind;
    state.in_x = x;
    state.in_y = y;
}

// Any event that reached us should refresh the readout even if the handler
// itself had nothing to redraw, so the counter visibly advances on every click.
fn force_repaint(outcome: EventOutcome) -> EventOutcome {
    match outcome {
        EventOutcome::Idle => EventOutcome::Repaint,
        other => other,
    }
}
