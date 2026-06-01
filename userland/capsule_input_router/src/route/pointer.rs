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

use nonos_libc::{InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL};

use crate::clients::{compositor, wm};
use crate::state::Context;

use super::deliver::deliver_one;

pub fn route_pointer(ctx: &mut Context, event: &InputEvent) -> u32 {
    refresh_display(ctx);
    let (x, y) = ctx.cursor.apply(event);
    let rid = ctx.issue_request_id();
    let Some(target) = wm::query_topmost(&mut ctx.wm_port, rid, x, y) else {
        ctx.record(0);
        return 0;
    };
    if target.owner_pid == 0 {
        ctx.record(0);
        return 0;
    }
    if event.kind == INPUT_KIND_BUTTON_DOWN {
        let rid = ctx.issue_request_id();
        if wm::route_focus(ctx.wm_port, rid, target) {
            ctx.focus_pid = target.owner_pid;
        }
    }
    let mut routed = *event;
    if routed.kind == INPUT_KIND_POINTER_REL {
        routed.kind = INPUT_KIND_POINTER_ABS;
        routed.delta_x = 0;
        routed.delta_y = 0;
    }
    routed.x = target.local_x as i32;
    routed.y = target.local_y as i32;
    if !ctx.subscriptions.allows(target.owner_pid, routed.kind) {
        ctx.record(0);
        return 0;
    }
    let delivered = deliver_one(target.owner_pid, &routed);
    ctx.record(delivered);
    delivered
}

fn refresh_display(ctx: &mut Context) {
    if ctx.cursor.configured {
        return;
    }
    let rid = ctx.issue_request_id();
    if let Some((width, height)) = compositor::display_size(&mut ctx.compositor_port, rid) {
        ctx.cursor.configure(width, height);
    }
}
