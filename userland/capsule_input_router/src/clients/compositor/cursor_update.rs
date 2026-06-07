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

use core::sync::atomic::{AtomicU32, Ordering};

use super::super::wire::{lookup_pid, send_to};
use super::constants::{MAGIC, OP_CURSOR_UPDATE, SERVICE};

static COMPOSITOR_PID: AtomicU32 = AtomicU32::new(0);

pub fn cursor_update(_port_slot: &mut u32, request_id: u32, x: u32, y: u32, visible: bool) -> bool {
    let mut pid = COMPOSITOR_PID.load(Ordering::Relaxed);
    if pid == 0 {
        match lookup_pid(SERVICE) {
            Some(found) => {
                COMPOSITOR_PID.store(found, Ordering::Relaxed);
                pid = found;
            }
            None => return false,
        }
    }
    let mut payload = [0u8; 16];
    payload[0..4].copy_from_slice(&x.to_le_bytes());
    payload[4..8].copy_from_slice(&y.to_le_bytes());
    payload[8..12].copy_from_slice(&(visible as u32).to_le_bytes());
    send_to(pid, MAGIC, OP_CURSOR_UPDATE, request_id, &payload)
}
