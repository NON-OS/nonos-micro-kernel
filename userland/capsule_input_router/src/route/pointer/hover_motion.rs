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

use nonos_libc::{InputEvent, INPUT_KIND_POINTER_ABS};

use crate::state::{Context, Hover};

use super::super::deliver::deliver_one;
use super::shell_pid::shell_pid;
use super::topmost_target::topmost_target;

const REQUERY_EVERY: u32 = 4;

pub(super) fn hover_motion(ctx: &mut Context, event: &InputEvent, x: u32, y: u32) -> u32 {
    if let Some(hover) = ctx.hover {
        if hover.contains(x, y) {
            return deliver_local(ctx, hover, event, x, y);
        }
        send_leave(ctx, hover.pid, event);
        ctx.hover = None;
    }
    ctx.hover_tick = ctx.hover_tick.wrapping_add(1);
    if ctx.hover_tick % REQUERY_EVERY != 0 {
        return 0;
    }
    match topmost_target(ctx, x, y) {
        Some(t) if t.owner_pid != shell_pid(ctx) => {
            let hover = Hover { pid: t.owner_pid, x: t.win_x, y: t.win_y, w: t.win_w, h: t.win_h };
            ctx.hover = Some(hover);
            deliver_local(ctx, hover, event, x, y)
        }
        _ => 0,
    }
}

fn deliver_local(ctx: &mut Context, hover: Hover, event: &InputEvent, x: u32, y: u32) -> u32 {
    let mut routed = *event;
    routed.kind = INPUT_KIND_POINTER_ABS;
    routed.x = (x - hover.x) as i32;
    routed.y = (y - hover.y) as i32;
    routed.delta_x = 0;
    routed.delta_y = 0;
    if !ctx.subscriptions.allows(hover.pid, routed.kind) {
        return 0;
    }
    let delivered = deliver_one(hover.pid, &routed);
    if delivered == 0 {
        ctx.forget_pid(hover.pid);
        ctx.hover = None;
    }
    delivered
}

fn send_leave(ctx: &mut Context, pid: u32, event: &InputEvent) {
    let mut routed = *event;
    routed.kind = INPUT_KIND_POINTER_ABS;
    routed.x = -1;
    routed.y = -1;
    routed.delta_x = 0;
    routed.delta_y = 0;
    if ctx.subscriptions.allows(pid, routed.kind) && deliver_one(pid, &routed) == 0 {
        ctx.forget_pid(pid);
    }
}
