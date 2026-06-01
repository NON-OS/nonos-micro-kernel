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
    InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN,
    INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH,
    INPUT_KIND_WHEEL,
};

use crate::state::Context;

use super::deliver::deliver_one;
use super::{keyboard, pointer};

pub fn route_event(ctx: &mut Context, event: &InputEvent) -> u32 {
    if let Some(holder) = ctx.grabs.holder_for(event.kind) {
        let n = deliver_one(holder, event);
        ctx.record(n);
        return n;
    }
    if is_pointer(event.kind) {
        return pointer::route_pointer(ctx, event);
    }
    if is_keyboard(event.kind) {
        return keyboard::route_keyboard(ctx, event);
    }
    let mut total = 0u32;
    for pid in ctx.subscriptions.match_kind(event.kind) {
        total += deliver_one(pid, event);
    }
    ctx.record(total);
    total
}

fn is_keyboard(kind: u16) -> bool {
    kind == INPUT_KIND_KEY_DOWN || kind == INPUT_KIND_KEY_UP
}

fn is_pointer(kind: u16) -> bool {
    matches!(
        kind,
        INPUT_KIND_POINTER_REL
            | INPUT_KIND_POINTER_ABS
            | INPUT_KIND_WHEEL
            | INPUT_KIND_BUTTON_DOWN
            | INPUT_KIND_BUTTON_UP
            | INPUT_KIND_TOUCH
    )
}
