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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, KEY_ESC};

use crate::wallet::state::State;

// Repaint only when a handler actually changed state; a click or key that hits
// nothing returns Idle and the dense screen is not re-rendered. This keeps the
// UI responsive — every stray click no longer forces a full 1280x800 repaint.
pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.kind {
        InputKind::KeyDown if event.code == KEY_ESC => EventOutcome::Close,
        InputKind::KeyDown => super::on_key::on_key(state, event.code),
        InputKind::ButtonDown => super::on_pointer::on_pointer(state, event.x, event.y),
        _ => EventOutcome::Idle,
    }
}
