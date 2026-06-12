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

use nonos_libc::mk_service_lookup;

use crate::state::Context;

const INPUT_ROUTER: &[u8] = b"input_router";

fn lookup_router_pid() -> u32 {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(INPUT_ROUTER.as_ptr(), INPUT_ROUTER.len(), &mut port, &mut pid);
    if rc >= 0 && port != 0 {
        pid
    } else {
        0
    }
}

pub fn is_input_router(ctx: &mut Context, sender_pid: u32) -> bool {
    if ctx.input_router_pid != sender_pid {
        ctx.input_router_pid = lookup_router_pid();
    }
    ctx.input_router_pid != 0 && ctx.input_router_pid == sender_pid
}
