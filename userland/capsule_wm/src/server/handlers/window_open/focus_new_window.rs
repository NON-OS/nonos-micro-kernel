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

use crate::compositor_client::push_focus_set;
use crate::state::Context;

pub(super) fn focus_new_window(ctx: &mut Context, sender_pid: u32, window_id: u32) -> bool {
    let unchanged = matches!(
        ctx.focus.current(),
        Some(f) if f.owner_pid == sender_pid && f.window_id == window_id
    );
    if unchanged {
        return true;
    }
    let rid = ctx.issue_request_id();
    if push_focus_set(ctx.compositor_port, rid, sender_pid).is_err() {
        return false;
    }
    ctx.focus.set(sender_pid, window_id)
}
