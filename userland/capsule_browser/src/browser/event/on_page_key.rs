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
    EventOutcome, InputEvent, KEY_DOWN, KEY_END, KEY_HOME, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_UP,
};

use crate::browser::event::scroll_by;
use crate::browser::paint::document::VIEW_H;
use crate::browser::state::State;

pub fn on_page_key(state: &mut State, event: InputEvent) -> EventOutcome {
    if let Some(id) = state.focus {
        return super::field_key::field_key(state, id, event);
    }
    let page = VIEW_H as i32 - 40;
    match event.code {
        KEY_UP => scroll_by::scroll_by(state, -40),
        KEY_DOWN => scroll_by::scroll_by(state, 40),
        KEY_PAGE_UP => scroll_by::scroll_by(state, -page),
        KEY_PAGE_DOWN | 0x20 => scroll_by::scroll_by(state, page),
        KEY_HOME => state.scroll = 0,
        KEY_END => scroll_by::scroll_by(state, i32::MAX),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
