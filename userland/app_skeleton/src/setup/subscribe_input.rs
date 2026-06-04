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

use crate::clients::input_router;
use nonos_libc::mk_yield;

use super::request_id::bump;

const SUBSCRIBE_ATTEMPTS: usize = 4;

pub(super) fn subscribe_input(
    port: u32,
    request_id: &mut u32,
    kind_mask: u32,
) -> Result<(), &'static str> {
    for _ in 0..SUBSCRIBE_ATTEMPTS {
        let rid = bump(request_id);
        if input_router::subscribe(port, rid, kind_mask).is_ok() {
            return Ok(());
        }
        mk_yield();
    }
    Err("input subscribe deferred")
}
