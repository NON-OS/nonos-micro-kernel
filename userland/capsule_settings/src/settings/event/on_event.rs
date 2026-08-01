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

use crate::settings::state::{cursor_down, cursor_up, State};

use super::on_event_browsing::on_event_browsing;
use super::on_event_editing::on_event_editing;
use super::on_event_wifi::on_event_wifi;
use super::on_pointer::on_pointer;

// Rows moved per wheel notch.
const WHEEL_STEP: u32 = 3;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        return on_pointer(state, event.x, event.y);
    }
    // Wheel events were delivered and dropped. Moving the cursor rather than a
    // scroll offset keeps the selection and the view together, and reuses the
    // clamping the arrow keys already do.
    if event.kind == InputKind::Wheel && !state.editing {
        let steps = event.delta_y.unsigned_abs().min(10) * WHEEL_STEP;
        if steps == 0 {
            return EventOutcome::Idle;
        }
        for _ in 0..steps {
            if event.delta_y > 0 {
                cursor_up(state);
            } else {
                cursor_down(state);
            }
        }
        return EventOutcome::Repaint;
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if state.editing {
        return on_event_editing(state, event.code);
    }
    if state.wifi_active {
        return on_event_wifi(state, event.code);
    }
    on_event_browsing(state, event.code)
}
