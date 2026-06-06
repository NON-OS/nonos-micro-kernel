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

use super::super::wire::call;
use super::constants::{MAGIC, OP_ROUTE_FOCUS};
use super::types::Target;

pub fn route_focus(port: u32, request_id: u32, target: Target) -> bool {
    let mut req = [0u8; 8];
    req[0..4].copy_from_slice(&target.owner_pid.to_le_bytes());
    req[4..8].copy_from_slice(&target.window_id.to_le_bytes());
    call(port, MAGIC, OP_ROUTE_FOCUS, request_id, &req, &mut []).map(|s| s == 0).unwrap_or(false)
}
