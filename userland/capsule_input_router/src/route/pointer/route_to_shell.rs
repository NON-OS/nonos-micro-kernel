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

use nonos_libc::{InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_TOUCH};

use crate::state::Context;

use super::super::deliver::deliver_one;
use super::shell_pid::shell_pid;

pub(super) fn route_to_shell(ctx: &mut Context, event: &InputEvent, x: u32, y: u32) -> u32 {
    if !matches!(event.kind, INPUT_KIND_BUTTON_DOWN | INPUT_KIND_TOUCH) {
        return 0;
    }
    if shell_pid(ctx) == 0 {
        return 0;
    }
    let mut routed = *event;
    routed.x = x as i32;
    routed.y = y as i32;
    let pid = ctx.shell_pid;
    let delivered = deliver_one(pid, &routed);
    if delivered == 0 {
        ctx.forget_pid(pid);
    }
    delivered
}
