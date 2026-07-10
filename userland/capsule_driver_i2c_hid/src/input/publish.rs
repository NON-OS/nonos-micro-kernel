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

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_REL, INPUT_KIND_WHEEL,
};

use super::post::post;
use super::sample::MouseSample;
use crate::state::State;

pub fn publish(state: &mut State, sample: MouseSample) {
    let mut ok = true;
    if sample.dx != 0 || sample.dy != 0 {
        ok &= post(INPUT_KIND_POINTER_REL, 0, sample.dx as i32, sample.dy as i32);
    }
    if sample.wheel != 0 {
        ok &= post(INPUT_KIND_WHEEL, 0, 0, sample.wheel as i32);
    }
    let changed = (state.last_buttons ^ sample.buttons) & 0x1f;
    for bit in 0..5u8 {
        let mask = 1u8 << bit;
        if changed & mask != 0 {
            let kind = if sample.buttons & mask != 0 {
                INPUT_KIND_BUTTON_DOWN
            } else {
                INPUT_KIND_BUTTON_UP
            };
            ok &= post(kind, u32::from(bit) + 1, 0, 0);
        }
    }
    state.last_buttons = sample.buttons;
    if !ok {
        state.post_failures = state.post_failures.wrapping_add(1);
    }
}
