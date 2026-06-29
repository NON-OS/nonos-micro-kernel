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

const FIRST_WAIT: u32 = 4000;
const IDLE_AFTER_DATA: u32 = 64;
const YIELD_BURST: u32 = 400;

pub fn recv_all(sockets_port: u32, handle: u32, max: usize) -> Result<Vec<u8>, ()> {
    let mut out = Vec::new();
    let mut idle = 0u32;
    loop {
        let mut chunk = [0u8; 4096];
        match super::socket_recv::socket_recv(sockets_port, handle, &mut chunk) {
            Ok(n) if n > 0 => {
                idle = 0;
                out.extend_from_slice(&chunk[..n]);
                if out.len() >= max {
                    break;
                }
            }
            _ => {
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
    if out.is_empty() { Err(()) } else { Ok(out) }
}
