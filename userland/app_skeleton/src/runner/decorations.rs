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
use nonos_toolkit::decorations::{content_rect, hit_test, DecorationHit};

pub(super) fn normalize(mut event: InputEvent) -> InputEvent {
    if event.kind == InputKind::Touch {
        event.kind = InputKind::ButtonDown;
    }
    event
}

pub(super) fn handle(
    width: u32,
    height: u32,
    maximized: bool,
    event: InputEvent,
) -> Option<EventOutcome> {
    if event.kind != InputKind::ButtonDown || event.x < 0 || event.y < 0 {
        return None;
    }
    match hit_test(width, height, maximized, event.x as u32, event.y as u32) {
        DecorationHit::CloseButton => Some(EventOutcome::Close),
        DecorationHit::MinimizeButton => Some(EventOutcome::Minimize),
        DecorationHit::MaximizeButton => Some(EventOutcome::Maximize),
        _ => None,
    }
}

pub(super) fn to_content(
    width: u32,
    height: u32,
    maximized: bool,
    mut event: InputEvent,
) -> InputEvent {
    let c = content_rect(width, height, maximized);
    event.x -= c.x as i32;
    event.y -= c.y as i32;
    event
}
