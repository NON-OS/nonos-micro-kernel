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

use nonos_libc::{mk_time_millis, mk_yield};

use crate::clients::tcp;

const ESTABLISHED: u8 = 3;
const CLOSED: u8 = 0xFF;
const DEADLINE_MS: i64 = 8000;

// A transport connect only starts the handshake; sending before the three-way
// completes fails. Poll the connection state (each poll also drives net.core's
// interface) until it establishes, the peer closes, or the deadline passes.
pub(super) fn wait_established(port: u32, handle: u32) -> bool {
    let start = mk_time_millis();
    loop {
        match tcp::state(port, handle) {
            Ok(ESTABLISHED) => return true,
            Ok(CLOSED) | Err(_) => return false,
            Ok(_) => {}
        }
        if mk_time_millis().wrapping_sub(start) > DEADLINE_MS {
            return false;
        }
        for _ in 0..16 {
            mk_yield();
        }
    }
}
