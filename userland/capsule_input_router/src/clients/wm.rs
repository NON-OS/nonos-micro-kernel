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

use super::wire::{call, lookup_port, u32_at};

const SERVICE: &[u8] = b"wm";
const MAGIC: u32 = 0x4E57_4D50;
const OP_QUERY_TOPMOST: u16 = 0x000B;
const OP_ROUTE_FOCUS: u16 = 0x000C;
const OP_QUERY_FOCUS: u16 = 0x000D;

#[derive(Clone, Copy)]
pub struct Target {
    pub owner_pid: u32,
    pub window_id: u32,
    pub local_x: u32,
    pub local_y: u32,
}

pub fn query_topmost(port_slot: &mut u32, request_id: u32, x: u32, y: u32) -> Option<Target> {
    if *port_slot == 0 {
        *port_slot = lookup_port(SERVICE)?;
    }
    let mut req = [0u8; 8];
    req[0..4].copy_from_slice(&x.to_le_bytes());
    req[4..8].copy_from_slice(&y.to_le_bytes());
    let mut body = [0u8; 16];
    if call(*port_slot, MAGIC, OP_QUERY_TOPMOST, request_id, &req, &mut body).ok()? != 0 {
        *port_slot = 0;
        return None;
    }
    Some(Target {
        owner_pid: u32_at(&body, 0).ok()?,
        window_id: u32_at(&body, 4).ok()?,
        local_x: u32_at(&body, 8).ok()?,
        local_y: u32_at(&body, 12).ok()?,
    })
}

// Ask the WM, the focus authority, which pid currently holds keyboard focus.
// Returns None when nothing is focused or the call fails. Keyboard events are
// low frequency, so querying per event is cheaper than caching focus locally
// and keeps the router in sync with self-focus (launcher) and window close.
pub fn query_focus(port_slot: &mut u32, request_id: u32) -> Option<u32> {
    if *port_slot == 0 {
        *port_slot = lookup_port(SERVICE)?;
    }
    let mut body = [0u8; 8];
    if call(*port_slot, MAGIC, OP_QUERY_FOCUS, request_id, &[], &mut body).ok()? != 0 {
        *port_slot = 0;
        return None;
    }
    let owner_pid = u32_at(&body, 0).ok()?;
    if owner_pid == 0 {
        None
    } else {
        Some(owner_pid)
    }
}

pub fn route_focus(port: u32, request_id: u32, target: Target) -> bool {
    let mut req = [0u8; 8];
    req[0..4].copy_from_slice(&target.owner_pid.to_le_bytes());
    req[4..8].copy_from_slice(&target.window_id.to_le_bytes());
    call(port, MAGIC, OP_ROUTE_FOCUS, request_id, &req, &mut []).map(|s| s == 0).unwrap_or(false)
}
