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
    KEY_DOWN, KEY_END, KEY_ENTER, KEY_HOME, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_UP,
};

use super::super::state::{Screen, State};

// Arrow, page and home/end drive whichever list the active screen shows, each
// clamped by the count that screen's own geometry reported to paint. Returns
// false for a code this handler does not own so the caller can keep matching.
pub fn scroll_key(state: &mut State, code: u32) -> bool {
    let security = state.screen == Screen::Security;
    let (page, span) = if security {
        (state.alert_visible.max(1) as i32, state.alerts.len() as i32)
    } else {
        (state.visible.max(1) as i32, state.filtered().len() as i32)
    };
    let delta = match code {
        KEY_UP => -1,
        KEY_DOWN => 1,
        KEY_PAGE_UP => -page,
        KEY_PAGE_DOWN => page,
        KEY_HOME => -span,
        KEY_END => span,
        KEY_ENTER if security => {
            state.jump_to_alert();
            return true;
        }
        _ => return false,
    };
    if security {
        state.alert_move(delta);
    } else {
        state.move_selection(delta);
    }
    true
}
