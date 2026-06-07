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
use super::constants::{MAGIC, OP_DISPLAY_INFO, SERVICE};

pub fn display_size(port_slot: &mut u32, request_id: u32) -> Option<(u32, u32)> {
    if *port_slot == 0 {
        *port_slot = lookup_port(SERVICE)?;
    }
    let mut body = [0u8; 16];
    if call(*port_slot, MAGIC, OP_DISPLAY_INFO, request_id, &[], &mut body).ok()? != 0 {
        *port_slot = 0;
        return None;
    }
    let width = u32_at(&body, 0).ok()?;
    let height = u32_at(&body, 4).ok()?;
    if width == 0 || height == 0 {
        None
    } else {
        Some((width, height))
    }
}
