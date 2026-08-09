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

pub fn on_event(event: InputEvent, clicks: &mut u32) -> EventOutcome {
    match event.kind {
        InputKind::ButtonDown => {
            *clicks = clicks.saturating_add(1);
            EventOutcome::Repaint
        }
        InputKind::KeyDown if event.code == KEY_ESC => EventOutcome::Close,
        _ => EventOutcome::Idle,
    }
}
