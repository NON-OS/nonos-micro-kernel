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

use nonos_app_skeleton::{EventOutcome, InputEvent};

use crate::browser::paint::home_page;
use crate::browser::state::State;

pub fn on_home_click(state: &mut State, event: InputEvent) -> EventOutcome {
    if home_page::search_bar_hit(event.x, event.y) {
        state.address_focused = true;
        return EventOutcome::Repaint;
    }
    match home_page::shortcut_at(event.x, event.y) {
        Some(url) => {
            state.address = url.into();
            state.pending_nav = Some(url.into());
            EventOutcome::Repaint
        }
        None => EventOutcome::Idle,
    }
}
