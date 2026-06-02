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

use crate::app::EventOutcome;
use crate::input::{InputEvent, InputKind};
use nonos_toolkit::decorations::{hit_test, DecorationHit};

pub(super) fn normalize(mut event: InputEvent) -> InputEvent {
    if event.kind == InputKind::Touch {
        event.kind = InputKind::ButtonDown;
    }
    event
}

pub(super) fn handle(width: u32, event: InputEvent) -> Option<EventOutcome> {
    if event.kind != InputKind::ButtonDown || event.x < 0 || event.y < 0 {
        return None;
    }
    if hit_test(width, event.x as u32, event.y as u32) == DecorationHit::CloseButton {
        return Some(EventOutcome::Close);
    }
    None
}
