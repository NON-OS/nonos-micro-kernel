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

use crate::input_router_client;
use crate::state::Context;

pub fn retry_input_subscription(ctx: &mut Context) {
    if ctx.input_ready || ctx.input_router_port == 0 {
        return;
    }
    let rid = ctx.issue_request_id();
    if input_router_client::subscribe(ctx.input_router_port, rid, ctx.input_kind_mask).is_ok() {
        ctx.input_ready = true;
    }
}
