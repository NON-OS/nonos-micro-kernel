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

use crate::browser::event::{on_button, on_key, on_page_key, scroll_by};
use crate::browser::state::{State, View};

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.kind {
        InputKind::ButtonDown => on_button::on_button(state, event),
        InputKind::Wheel => {
            scroll_by::scroll_by(state, -event.delta_y * 60);
            EventOutcome::Repaint
        }
        InputKind::KeyDown if state.address_focused => on_key::on_key(state, event),
        InputKind::KeyDown if matches!(state.view, View::Page) => {
            on_page_key::on_page_key(state, event)
        }
        _ => EventOutcome::Idle,
    }
}
