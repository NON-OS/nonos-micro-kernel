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

use super::super::wire::{call, lookup_port, u32_at};
use super::constants::{MAGIC, OP_QUERY_TOPMOST, SERVICE};
use super::types::Target;

pub fn query_topmost(port_slot: &mut u32, request_id: u32, x: u32, y: u32) -> Option<Target> {
    if *port_slot == 0 {
        *port_slot = lookup_port(SERVICE)?;
    }
    let mut req = [0u8; 8];
    req[0..4].copy_from_slice(&x.to_le_bytes());
    req[4..8].copy_from_slice(&y.to_le_bytes());
    let mut body = [0u8; 32];
    if call(*port_slot, MAGIC, OP_QUERY_TOPMOST, request_id, &req, &mut body).ok()? != 0 {
        *port_slot = 0;
        return None;
    }
    Some(Target {
        owner_pid: u32_at(&body, 0).ok()?,
        window_id: u32_at(&body, 4).ok()?,
        local_x: u32_at(&body, 8).ok()?,
        local_y: u32_at(&body, 12).ok()?,
        win_x: u32_at(&body, 16).ok()?,
        win_y: u32_at(&body, 20).ok()?,
        win_w: u32_at(&body, 24).ok()?,
        win_h: u32_at(&body, 28).ok()?,
    })
}
