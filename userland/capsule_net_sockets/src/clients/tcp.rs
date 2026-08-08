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

use super::envelope::{call, call_t};

const MAGIC: u32 = 0x4E54_4350;
const CONNECT_TIMEOUT_MS: u64 = 9000;
const LISTEN: u16 = 2;
const CONNECT: u16 = 3;
const ACCEPT: u16 = 4;
const RECV: u16 = 6;
const CLOSE: u16 = 7;

pub fn listen(port: u32, local: u16) -> Result<u32, u16> {
    call_handle(port, LISTEN, &local.to_le_bytes())
}

pub fn connect(port: u32, dst: [u8; 4], dst_port: u16) -> Result<u32, u16> {
    let mut body = [0u8; 6];
    body[0..4].copy_from_slice(&dst);
    body[4..6].copy_from_slice(&dst_port.to_le_bytes());
    let mut out = [0u8; 4];
    if call_t(port, MAGIC, CONNECT, &body, &mut out, CONNECT_TIMEOUT_MS)? != 4 {
        return Err(4);
    }
    Ok(u32::from_le_bytes(out))
}

pub fn accept(port: u32, handle: u32) -> Result<u32, u16> {
    call_handle(port, ACCEPT, &handle.to_le_bytes())
}

pub use super::tcp_send::send;

pub fn recv(port: u32, handle: u32, out: &mut [u8]) -> Result<usize, u16> {
    // State how much can be held. A read takes bytes out of the socket, so
    // whatever net.tcp sends back that does not fit here is lost rather than
    // kept for the next call.
    let mut body = [0u8; 8];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..8].copy_from_slice(&(out.len() as u32).to_le_bytes());
    call(port, MAGIC, RECV, &body, out)
}

pub fn close(port: u32, handle: u32) -> Result<(), u16> {
    call(port, MAGIC, CLOSE, &handle.to_le_bytes(), &mut []).map(|_| ())
}

pub use super::tcp_status::{poll, state};

fn call_handle(port: u32, op: u16, body: &[u8]) -> Result<u32, u16> {
    let mut out = [0u8; 4];
    if call(port, MAGIC, op, body, &mut out)? != 4 {
        return Err(4);
    }
    Ok(u32::from_le_bytes(out))
}
