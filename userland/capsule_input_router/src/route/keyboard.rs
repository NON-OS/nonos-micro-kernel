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

use crate::state::Context;

use super::deliver::deliver_one;

pub fn route_keyboard(ctx: &mut Context, event: &InputEvent) -> u32 {
    let pid = ctx.focus_pid;
    if pid == 0 || !ctx.subscriptions.allows(pid, event.kind) {
        ctx.record(0);
        return 0;
    }
    let delivered = deliver_one(pid, event);
    ctx.record(delivered);
    delivered
}
