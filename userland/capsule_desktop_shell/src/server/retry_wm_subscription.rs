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

use crate::state::Context;
use crate::wm_client;

pub fn retry_wm_subscription(ctx: &mut Context) {
    if ctx.wm_notify_ready || ctx.wm_port == 0 {
        return;
    }
    let rid = ctx.issue_request_id();
    if wm_client::lifecycle_subscribe(ctx.wm_port, rid).is_ok() {
        ctx.wm_notify_ready = true;
    }
}
