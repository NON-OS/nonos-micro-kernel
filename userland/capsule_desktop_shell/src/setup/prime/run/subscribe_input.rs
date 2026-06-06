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

pub fn subscribe_input(ctx: &mut Context, input_router_port: u32) {
    if super::subscribe_input_router::subscribe_input_router(
        input_router_port,
        &mut ctx.next_request_id,
        super::input_mask::SHELL_INPUT_MASK,
    )
    .is_ok()
    {
        ctx.input_ready = true;
    }
}
