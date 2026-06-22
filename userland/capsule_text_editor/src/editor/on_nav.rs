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
    EventOutcome, KEY_DOWN, KEY_END, KEY_HOME, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_UP,
};

use super::follow_end::follow_end;
use super::scroll_down::scroll_down;
use super::scroll_up::scroll_up;
use super::state::State;

pub(super) fn on_nav(state: &mut State, code: u32, rows: u32) -> Option<EventOutcome> {
    match code {
        KEY_UP => scroll_up(state, 1),
        KEY_DOWN => scroll_down(state, 1, rows),
        KEY_PAGE_UP => scroll_up(state, rows.saturating_sub(1).max(1)),
        KEY_PAGE_DOWN => scroll_down(state, rows.saturating_sub(1).max(1), rows),
        KEY_HOME => state.scroll_line = 0,
        KEY_END => follow_end(state, rows),
        _ => return None,
    }
    Some(EventOutcome::Repaint)
}
