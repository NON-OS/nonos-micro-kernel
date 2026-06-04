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

use nonos_libc::{InputEvent, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH};

use crate::state::Context;

use super::super::deliver::deliver_one;
use super::shell_pid::shell_pid;

pub(super) fn mirror_shell_pointer(ctx: &mut Context, event: &InputEvent, x: u32, y: u32) -> u32 {
    if !matches!(event.kind, INPUT_KIND_POINTER_REL | INPUT_KIND_POINTER_ABS | INPUT_KIND_TOUCH) {
        return 0;
    }
    let pid = shell_pid(ctx);
    if pid == 0 || !ctx.subscriptions.allows(pid, INPUT_KIND_POINTER_ABS) {
        return 0;
    }
    let mut routed = *event;
    routed.kind = INPUT_KIND_POINTER_ABS;
    routed.x = x as i32;
    routed.y = y as i32;
    routed.delta_x = 0;
    routed.delta_y = 0;
    let delivered = deliver_one(pid, &routed);
    if delivered == 0 {
        ctx.forget_pid(pid);
    }
    delivered
}
