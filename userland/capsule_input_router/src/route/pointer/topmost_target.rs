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

use crate::clients::wm;
use crate::state::Context;

pub(super) fn topmost_target(ctx: &mut Context, x: u32, y: u32) -> Option<wm::Target> {
    let rid = ctx.issue_request_id();
    wm::query_topmost(&mut ctx.wm_port, rid, x, y).filter(|target| target.owner_pid != 0)
}
