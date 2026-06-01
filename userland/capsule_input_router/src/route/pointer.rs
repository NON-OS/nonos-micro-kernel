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

use crate::clients::{compositor, wire, wm};
use crate::state::Context;

use super::deliver::deliver_one;

const DESKTOP_SHELL_SERVICE: &[u8] = b"desktop_shell";

pub fn route_pointer(ctx: &mut Context, event: &InputEvent) -> u32 {
    refresh_display(ctx);
    let (x, y) = ctx.cursor.apply(event);
    let rid = ctx.issue_request_id();
    let target = match wm::query_topmost(&mut ctx.wm_port, rid, x, y) {
        Some(t) if t.owner_pid != 0 => t,
        _ => return route_to_shell(ctx, event, x, y),
    };
    // Shell-owned chrome (launcher dock, taskbar) registers as topmost WM
    // windows but must not steal focus or it would black-hole the keyboard,
    // and the shell hit-tests its own layout in absolute screen coordinates.
    // Route these like the bare-desktop fallback: absolute coords, focus held.
    if target.owner_pid == shell_pid(ctx) {
        return route_to_shell(ctx, event, x, y);
    }
    if event.kind == INPUT_KIND_BUTTON_DOWN {
        let rid = ctx.issue_request_id();
        let _ = wm::route_focus(ctx.wm_port, rid, target);
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

// Resolve and cache the desktop shell pid. Reset to 0 on a failed delivery so a
// shell respawn is picked up on the next click.
fn shell_pid(ctx: &mut Context) -> u32 {
    if ctx.shell_pid == 0 {
        ctx.shell_pid = wire::lookup_pid(DESKTOP_SHELL_SERVICE).unwrap_or(0);
    }
    ctx.shell_pid
}

// Deliver desktop-chrome clicks to the shell in absolute screen coordinates.
// Covers two cases with one contract: bare-desktop clicks (no WM window under
// the cursor) and clicks on shell-owned chrome WM windows. The shell receives a
// single coordinate space and never gains focus, so the launched app keeps the
// keyboard. Only button presses are forwarded; bare movement needs no delivery.
fn route_to_shell(ctx: &mut Context, event: &InputEvent, x: u32, y: u32) -> u32 {
    if event.kind != INPUT_KIND_BUTTON_DOWN {
        ctx.record(0);
        return 0;
    }
    if shell_pid(ctx) == 0 {
        ctx.record(0);
        return 0;
    }
    let mut routed = *event;
    routed.x = x as i32;
    routed.y = y as i32;
    let delivered = deliver_one(ctx.shell_pid, &routed);
    if delivered == 0 {
        ctx.shell_pid = 0;
    }
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
