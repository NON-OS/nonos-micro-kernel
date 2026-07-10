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

//! Post the actions from a touch gesture as kernel input events. Absolute moves
//! go in the event's x/y fields (the router scales them), the wheel and buttons
//! as their own events.

use nonos_libc::{
    mk_input_event_post, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP,
    INPUT_KIND_POINTER_ABS, INPUT_KIND_WHEEL,
};

use super::gesture::TouchActions;
use crate::state::State;

pub fn publish_touch(state: &mut State, act: &TouchActions) {
    let mut ok = true;
    if let Some((x, y)) = act.move_to {
        ok &= post(INPUT_KIND_POINTER_ABS, 0, x as i32, y as i32, 0);
    }
    if act.wheel != 0 {
        ok &= post(INPUT_KIND_WHEEL, 0, 0, 0, act.wheel);
    }
    if act.button_down {
        ok &= post(INPUT_KIND_BUTTON_DOWN, 1, 0, 0, 0);
    }
    if act.button_up {
        ok &= post(INPUT_KIND_BUTTON_UP, 1, 0, 0, 0);
    }
    if !ok {
        state.post_failures = state.post_failures.wrapping_add(1);
    }
}

fn post(kind: u16, code: u32, x: i32, y: i32, wheel: i32) -> bool {
    let ev = InputEvent { kind, flags: 0, code, x, y, delta_x: 0, delta_y: wheel, timestamp_ns: 0 };
    mk_input_event_post(&ev) >= 0
}
