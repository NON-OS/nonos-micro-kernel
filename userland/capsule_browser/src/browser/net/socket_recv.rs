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

use alloc::vec;

use super::constants::{OP_RECV, SOCKETS_MAGIC};

const RECV_TIMEOUT_MS: u64 = 200;

pub fn socket_recv(sockets_port: u32, handle: u32, out: &mut [u8]) -> Result<usize, ()> {
    if super::mixnet::is_on() {
        return super::mixnet::recv(out);
    }
    let mut body = [0u8; 4];
    let mut rx = vec![0u8; out.len().saturating_add(20)];
    body.copy_from_slice(&handle.to_le_bytes());
    let n =
        super::call::call_t(sockets_port, SOCKETS_MAGIC, OP_RECV, &body, &mut rx, RECV_TIMEOUT_MS)?;
    if n < 20 {
        return Err(());
    }
    let payload = u32::from_le_bytes([rx[16], rx[17], rx[18], rx[19]]) as usize;
    let copy_len = core::cmp::min(core::cmp::min(payload, n - 20), out.len());
    out[..copy_len].copy_from_slice(&rx[20..20 + copy_len]);
    Ok(copy_len)
}
