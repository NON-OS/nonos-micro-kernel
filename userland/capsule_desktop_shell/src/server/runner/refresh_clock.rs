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

use crate::compositor_client::push_damage_commit;
use crate::render::{menubar_rect, paint_chrome, paint_status};
use crate::state::Context;

pub(super) fn refresh_clock(ctx: &mut Context) {
    paint_chrome(ctx);
    paint_status(ctx);
    let r = menubar_rect(ctx.width);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
}
