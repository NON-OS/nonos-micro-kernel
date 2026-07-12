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

//! Arrow and paging keys move the caret; the view scrolls only as far as it
//! must to keep the caret visible. Holding Shift extends the selection instead
//! of collapsing it.

use nonos_app_skeleton::{
    EventOutcome, KEY_DOWN, KEY_END, KEY_HOME, KEY_LEFT, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_RIGHT,
    KEY_UP,
};

use super::follow_caret::follow_caret;
use super::state::State;

pub(super) fn on_nav(state: &mut State, code: u32, rows: u32, shift: bool) -> Option<EventOutcome> {
    let is_nav = matches!(
        code,
        KEY_LEFT | KEY_RIGHT | KEY_UP | KEY_DOWN | KEY_PAGE_UP | KEY_PAGE_DOWN | KEY_HOME | KEY_END
    );
    if !is_nav {
        return None;
    }
    if shift {
        state.begin_sel();
    } else {
        state.clear_sel();
    }
    let page = rows.saturating_sub(1).max(1);
    match code {
        KEY_LEFT => state.caret_left(),
        KEY_RIGHT => state.caret_right(),
        KEY_UP => state.caret_up_by(1),
        KEY_DOWN => state.caret_down_by(1),
        KEY_PAGE_UP => state.caret_up_by(page),
        KEY_PAGE_DOWN => state.caret_down_by(page),
        KEY_HOME => state.caret_home(),
        KEY_END => state.caret_end(),
        _ => {}
    }
    follow_caret(state, rows);
    Some(EventOutcome::Repaint)
}
