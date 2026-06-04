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

use nonos_libc::mk_yield;

use crate::compositor_client::push_damage_commit;
use crate::render::paint_chrome;
use crate::state::Context;

pub fn paint_initial(ctx: &mut Context) {
    for _ in 0..8 {
        paint_chrome(ctx);
        let rid = ctx.issue_request_id();
        if push_damage_commit(ctx.compositor_port, rid, 0, 0, ctx.width, ctx.height).is_ok() {
            break;
        }
        mk_yield();
    }
}
