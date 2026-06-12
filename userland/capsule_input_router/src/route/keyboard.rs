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

use nonos_libc::InputEvent;

use crate::clients::wm;
use crate::state::Context;

use super::deliver::deliver_one;
use super::pointer::shell_pid;

pub fn route_keyboard(ctx: &mut Context, event: &InputEvent) -> u32 {
    let rid = ctx.issue_request_id();
    let pid = match wm::query_focus(&mut ctx.wm_port, rid) {
        Some(focused) => {
            ctx.last_focus_pid = focused;
            focused
        }
        None if ctx.last_focus_pid != 0 => ctx.last_focus_pid,
        None => shell_pid(ctx),
    };
    if !ctx.subscriptions.allows(pid, event.kind) {
        ctx.record(0);
        return 0;
    }
    let delivered = deliver_one(pid, event);
    if delivered == 0 {
        ctx.forget_pid(pid);
    }
    ctx.record(delivered);
    delivered
}
