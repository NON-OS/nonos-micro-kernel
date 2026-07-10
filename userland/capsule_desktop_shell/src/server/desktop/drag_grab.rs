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

//! Hold the pointer for the duration of an icon drag. Without the grab, the
//! release routes to whatever window the cursor happens to be over, the shell
//! never sees BUTTON_UP, and the icon is left glued to the cursor.

use crate::state::Context;

// Pointer-motion, button-down and button-up kind bits (1 << kind).
const DRAG_KINDS: u32 = (1 << 3) | (1 << 5) | (1 << 6);

pub fn grab_drag(ctx: &mut Context) {
    let rid = ctx.issue_request_id();
    let _ = crate::input_router_client::grab(ctx.input_router_port, rid, DRAG_KINDS);
}

pub fn release_drag(ctx: &mut Context) {
    let rid = ctx.issue_request_id();
    let _ = crate::input_router_client::release_grab(ctx.input_router_port, rid);
}
