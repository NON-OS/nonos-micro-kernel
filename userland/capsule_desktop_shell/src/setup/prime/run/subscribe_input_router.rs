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
use nonos_libc::mk_yield;

const SUBSCRIBE_ATTEMPTS: usize = 4;

pub fn subscribe_input_router(
    port: u32,
    request_id: &mut u32,
    kind_mask: u32,
) -> Result<(), &'static str> {
    for _ in 0..SUBSCRIBE_ATTEMPTS {
        let rid = bump(request_id);
        if input_router_client::subscribe(port, rid, kind_mask).is_ok() {
            return Ok(());
        }
        mk_yield();
    }
    Err("input subscribe deferred")
}

fn bump(slot: &mut u32) -> u32 {
    let id = *slot;
    *slot = slot.wrapping_add(1).max(1);
    id
}
