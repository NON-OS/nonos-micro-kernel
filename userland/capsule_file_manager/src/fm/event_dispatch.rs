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

use super::event_browse::on_browse_key;
use super::event_click::on_click;
use super::event_mode::route;
use super::event_query;
use super::screen::Screen;
use super::state::{Mode, State};

// Rows moved per wheel notch, matching the editor.
const WHEEL_STEP: usize = 3;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::Wheel {
        return wheel(state, event);
    }
    if event.kind == InputKind::ButtonDown {
        if event.x < 0 || event.y < 0 {
            return EventOutcome::Idle;
        }
        return on_click(state, event.x as u32, event.y as u32);
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if state.screen == Screen::Search && matches!(state.mode, Mode::Browse) {
        return event_query::on_key(state, event);
    }
    if let Some(outcome) = route(state, event) {
        return outcome;
    }
    on_browse_key(state, event.code)
}

// Wheel events were delivered and dropped, so a long listing could only be
// walked with the arrow keys.
fn wheel(state: &mut State, event: InputEvent) -> EventOutcome {
    let rows = (event.delta_y.unsigned_abs() as usize).min(10) * WHEEL_STEP;
    if rows == 0 {
        return EventOutcome::Idle;
    }
    let max = state.entries.len().saturating_sub(state.view_rows.max(1));
    state.scroll = if event.delta_y > 0 {
        state.scroll.saturating_sub(rows)
    } else {
        (state.scroll + rows).min(max)
    };
    EventOutcome::Repaint
}
