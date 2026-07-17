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

use crate::browser::event::nav_history;
use crate::browser::paint::chrome::{self, Btn};
use crate::browser::state::{State, View};

pub fn on_toolbar(state: &mut State, event: InputEvent) -> EventOutcome {
    match chrome::toolbar_button_at(event.x, event.y) {
        Some(Btn::Home) => {
            state.view = View::Home;
            state.address.clear();
            EventOutcome::Repaint
        }
        Some(Btn::Reload) => {
            if !state.address.is_empty() {
                state.pending_nav = Some(state.address.clone());
            }
            EventOutcome::Repaint
        }
        Some(Btn::Url) => {
            state.address_focused = true;
            EventOutcome::Repaint
        }
        Some(Btn::Back) => nav_history::nav_history(state, -1),
        Some(Btn::Forward) => nav_history::nav_history(state, 1),
        Some(Btn::Menu) => {
            state.settings_open = !state.settings_open;
            EventOutcome::Repaint
        }
        None => {
            state.address_focused = false;
            EventOutcome::Idle
        }
    }
}
