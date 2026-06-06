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
use crate::protocol::NOTIFY_KIND_CLOSED;
use crate::state::Context;

pub(super) fn sweep_dead(ctx: &mut Context) {
    ctx.subscriptions.purge_dead();
    while let Some(window) = ctx.windows.remove_one_dead() {
        if matches!(
            ctx.focus.current(),
            Some(f) if f.owner_pid == window.owner_pid && f.window_id == window.window_id
        ) {
            let rid = ctx.issue_request_id();
            let _ = push_focus_set(ctx.compositor_port, rid, 0);
            ctx.focus.clear();
        }
        crate::server::notify_fanout::broadcast(
            ctx,
            NOTIFY_KIND_CLOSED,
            window.owner_pid,
            window.window_id,
            window.rect.x,
            window.rect.y,
        );
    }
}
