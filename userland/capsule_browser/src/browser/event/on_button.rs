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

use crate::browser::event::{on_home_click, on_page_click, on_toolbar};
use crate::browser::paint::chrome::TITLEBAR;
use crate::browser::paint::home_page::CONTENT_TOP;
use crate::browser::state::{State, View};

pub fn on_button(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.y < TITLEBAR as i32 {
        return EventOutcome::Idle;
    }
    // While the settings panel is open it takes every click: its buttons act,
    // and a click anywhere else closes it.
    if state.settings_open {
        return crate::browser::settings::on_click(state, event.x, event.y);
    }
    if event.y < CONTENT_TOP as i32 {
        return on_toolbar::on_toolbar(state, event);
    }
    match state.view {
        View::Home => on_home_click::on_home_click(state, event),
        View::Page => on_page_click::on_page_click(state, event),
    }
}
