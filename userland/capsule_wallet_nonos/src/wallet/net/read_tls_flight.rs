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

pub fn read_tls_flight(sockets_port: u32, handle: u32) -> Result<Vec<u8>, ()> {
    let mut out = Vec::new();
    for _ in 0..8 {
        let mut chunk = [0u8; 4096];
        let n = match super::socket_recv::socket_recv(sockets_port, handle, &mut chunk) {
            Ok(n) => n,
            Err(()) if !out.is_empty() => break,
            Err(()) => return Err(()),
        };
        if n == 0 {
            break;
        }
        out.extend_from_slice(&chunk[..n]);
        if super::super::tls13::server_finished_flight_ready(&out) {
            break;
        }
        if out.len() > 24 * 1024 {
            return Err(());
        }
    }
    Ok(out)
}
