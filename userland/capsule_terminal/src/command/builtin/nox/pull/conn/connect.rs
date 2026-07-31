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

use nonos_libc::mk_ipc_call_timeout;

use super::frame::{frame, lookup, status, HDR, OP_CONNECT};

const CONNECT_MS: u64 = 9000;

pub struct Conn {
    pub(super) port: u32,
    pub(super) handle: u32,
}

pub enum Rx {
    Data,
    Empty,
    Gone,
}

pub fn connect(ip: [u8; 4], port: u16) -> Option<Conn> {
    let svc = lookup()?;
    let mut body = [0u8; 6];
    body[..4].copy_from_slice(&ip);
    body[4..6].copy_from_slice(&port.to_le_bytes());
    let tx = frame(OP_CONNECT, &body);
    let mut rx = [0u8; HDR + 4];
    let n = mk_ipc_call_timeout(
        svc as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        CONNECT_MS,
    );
    if n < (HDR + 4) as i64 || status(&rx) != 0 {
        return None;
    }
    let handle = u32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]);
    Some(Conn { port: svc, handle })
}
