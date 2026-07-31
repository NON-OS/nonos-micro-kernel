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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::on_key::on_key;
use crate::term::state::State;

// Scrollback lines per wheel notch, matching the editor.
const WHEEL_STEP: usize = 3;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    // Wheel events were being delivered and dropped, so the scrollback could
    // only be reached with the keyboard.
    if event.kind == InputKind::Wheel {
        let lines = (event.delta_y.unsigned_abs() as usize).min(10) * WHEEL_STEP;
        if lines == 0 {
            return EventOutcome::Idle;
        }
        if event.delta_y > 0 {
            state.scrollback.scroll_up(lines);
        } else {
            state.scrollback.scroll_down(lines);
        }
        return EventOutcome::Repaint;
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    on_key(state, event)
}
