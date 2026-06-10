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

use nonos_toolkit::decorations::{hit_test, DecorationHit};

use crate::input::{InputEvent, InputKind};

const MENUBAR_H: i64 = 28;

// Motion arrives in the input router's frozen-origin frame, so the
// difference from the press point equals the cursor's screen delta
// even after this window has already moved mid-drag.
pub(super) struct DragState {
    pub active: bool,
    press_x: i32,
    press_y: i32,
    base_x: u32,
    base_y: u32,
}

impl DragState {
    pub(super) const fn new() -> Self {
        Self { active: false, press_x: 0, press_y: 0, base_x: 0, base_y: 0 }
    }
}

pub(super) fn handle(
    state: &mut DragState,
    width: u32,
    win_x: u32,
    win_y: u32,
    event: &InputEvent,
) -> Option<(u32, u32)> {
    match event.kind {
        InputKind::ButtonDown => {
            if event.x >= 0
                && event.y >= 0
                && hit_test(width, event.x as u32, event.y as u32) == DecorationHit::Titlebar
            {
                *state = DragState {
                    active: true,
                    press_x: event.x,
                    press_y: event.y,
                    base_x: win_x,
                    base_y: win_y,
                };
            }
            None
        }
        InputKind::ButtonUp => {
            state.active = false;
            None
        }
        InputKind::PointerAbs if state.active => {
            let nx = state.base_x as i64 + (event.x - state.press_x) as i64;
            let ny = state.base_y as i64 + (event.y - state.press_y) as i64;
            Some((nx.max(0) as u32, ny.max(MENUBAR_H) as u32))
        }
        _ => None,
    }
}
