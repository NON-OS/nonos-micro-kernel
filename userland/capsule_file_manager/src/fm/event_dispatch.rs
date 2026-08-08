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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, KEY_ENTER};

use super::event_browse::on_browse_key;
use super::event_grid::grid_select;
use super::event_mode::route;
use super::event_mouse::select_row;
use super::layout::SIDEBAR_W;
use super::refresh::refresh;
use super::state::{State, ViewKind};

// Rows moved per wheel notch, matching the editor.
const WHEEL_STEP: usize = 3;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    // Wheel events were delivered and dropped, so a long listing could only be
    // walked with the arrow keys.
    if event.kind == InputKind::Wheel {
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
        return EventOutcome::Repaint;
    }
    // A click in the PLACES sidebar navigates straight to that location and
    // must not fall through to row selection / open.
    if event.kind == InputKind::ButtonDown
        && event.x >= 0
        && event.y >= 0
        && (event.x as u32) < SIDEBAR_W
        && matches!(state.mode, super::state::Mode::Browse)
    {
        if let Some(path) = super::paint_sidebar::place_at(event.y as u32) {
            if state.prefix.as_str() != path {
                state.prefix = alloc::string::String::from(path);
                state.cursor = 0;
                state.scroll = 0;
                refresh(state);
            }
            return EventOutcome::Repaint;
        }
        return EventOutcome::Idle;
    }
    // Map a click to the cell/row of the active view before the open below runs.
    match state.view {
        ViewKind::Grid => grid_select(state, event),
        ViewKind::List => select_row(state, event),
    }
    if event.kind != InputKind::ButtonDown && !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if let Some(outcome) = route(state, event) {
        return outcome;
    }
    let code = if event.kind == InputKind::ButtonDown { KEY_ENTER } else { event.code };
    on_browse_key(state, code)
}
