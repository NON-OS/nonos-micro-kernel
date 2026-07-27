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

use alloc::vec::Vec;

// Bounded wait for the first response bytes. Kept generous enough for a live
// but slow RPC, low enough that a dead connection fails fast and the single
// per-tick read never stalls the UI for long.
const FIRST_WAIT: u32 = 1400;
const IDLE_AFTER_DATA: u32 = 48;
const YIELD_BURST: u32 = 400;

pub fn read_tls_flight(sockets_port: u32, handle: u32) -> Result<Vec<u8>, ()> {
    let mut out = Vec::new();
    let mut idle = 0u32;
    loop {
        let mut chunk = [0u8; 4096];
        match super::socket_recv::socket_recv(sockets_port, handle, &mut chunk) {
            Ok(n) if n > 0 => {
                idle = 0;
                out.extend_from_slice(&chunk[..n]);
                if super::super::tls13::server_finished_flight_ready(&out) {
                    break;
                }
                if out.len() > 24 * 1024 {
                    return Err(());
                }
            }
            _ => {
                if super::super::tls13::server_finished_flight_ready(&out) {
                    break;
                }
                idle += 1;
                let budget = if out.is_empty() { FIRST_WAIT } else { IDLE_AFTER_DATA };
                if idle >= budget {
                    break;
                }
                for _ in 0..YIELD_BURST {
                    nonos_libc::mk_yield();
                }
            }
        }
    }
    if out.is_empty() {
        Err(())
    } else {
        Ok(out)
    }
}
