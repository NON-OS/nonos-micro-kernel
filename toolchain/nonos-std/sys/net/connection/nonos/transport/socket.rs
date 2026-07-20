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

// Socket primitives over net.sockets: open a socket of a given kind, send and
// recv a bounded payload on a handle, and close it. The stream and datagram
// types build their operations on these.

use super::consts::{BODY, MAX_PAYLOAD, OP_CLOSE, OP_RECV, OP_SEND, OP_SOCKET};
use super::err::read_u32;
use super::ipc::sk;
use crate::io;
use crate::vec::Vec;

pub(crate) fn open_socket(kind: u16) -> io::Result<u32> {
    let mut body = [0u8; 4];
    body[0..2].copy_from_slice(&4u16.to_le_bytes());
    body[2..4].copy_from_slice(&kind.to_le_bytes());
    Ok(read_u32(&sk(OP_SOCKET, &body, 8)?, BODY))
}

pub(crate) fn send_on(handle: u32, buf: &[u8]) -> io::Result<usize> {
    let n = buf.len().min(MAX_PAYLOAD);
    let mut body = Vec::with_capacity(4 + n);
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&buf[..n]);
    sk(OP_SEND, &body, 0)?;
    Ok(n)
}

pub(crate) fn recv_on(handle: u32, buf: &mut [u8]) -> io::Result<usize> {
    let rx = sk(OP_RECV, &handle.to_le_bytes(), MAX_PAYLOAD)?;
    let data = &rx[BODY..];
    let n = data.len().min(buf.len());
    buf[..n].copy_from_slice(&data[..n]);
    Ok(n)
}

pub(crate) fn close(handle: u32) {
    let _ = sk(OP_CLOSE, &handle.to_le_bytes(), 0);
}
