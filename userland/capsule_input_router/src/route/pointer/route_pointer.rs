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
    InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_TOUCH, INPUT_KIND_WHEEL,
};

use crate::state::Context;

use super::mirror_shell_pointer::mirror_shell_pointer;
use super::refresh_display::refresh_display;
use super::route_to_shell::route_to_shell;
use super::route_to_window::route_to_window;
use super::shell_pid::shell_pid;
use super::topmost_target::topmost_target;

pub fn route_pointer(ctx: &mut Context, event: &InputEvent) -> u32 {
    refresh_display(ctx);
    let (x, y) = ctx.cursor.apply(event);
    ctx.cursor_x = x;
    ctx.cursor_y = y;
    ctx.cursor_dirty = true;
    let mut delivered = mirror_shell_pointer(ctx, event, x, y);
    if needs_hit_test(event.kind) {
        delivered += match topmost_target(ctx, x, y) {
            None => route_to_shell(ctx, event, x, y),
            Some(target) if target.owner_pid == shell_pid(ctx) => route_to_shell(ctx, event, x, y),
            Some(target) => route_to_window(ctx, event, target),
        };
    }
    ctx.record(delivered);
    delivered
}

fn needs_hit_test(kind: u16) -> bool {
    matches!(
        kind,
        INPUT_KIND_BUTTON_DOWN | INPUT_KIND_BUTTON_UP | INPUT_KIND_TOUCH | INPUT_KIND_WHEEL
    )
}
