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
use super::event_mode::route;
use super::event_mouse::select_row;
use super::state::State;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    select_row(state, event);
    if event.kind != InputKind::ButtonDown && !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if let Some(outcome) = route(state, event) {
        return outcome;
    }
    let code = if event.kind == InputKind::ButtonDown { KEY_ENTER } else { event.code };
    on_browse_key(state, code)
}
