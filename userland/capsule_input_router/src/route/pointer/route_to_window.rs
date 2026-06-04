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
    InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL,
    INPUT_KIND_TOUCH,
};

use crate::clients::wm;
use crate::state::Context;

use super::super::deliver::deliver_one;

pub(super) fn route_to_window(ctx: &mut Context, event: &InputEvent, target: wm::Target) -> u32 {
    if matches!(event.kind, INPUT_KIND_BUTTON_DOWN | INPUT_KIND_TOUCH) {
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
        return 0;
    }
    let delivered = deliver_one(target.owner_pid, &routed);
    if delivered == 0 {
        ctx.forget_pid(target.owner_pid);
    }
    delivered
}
