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


//! Bytes on and off a connected socket.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::call::call;
use super::ops::{OP_CLOSE, OP_RECV, OP_SEND};

/// One transfer. The server's reply buffer is the ceiling, not this.
const MAX_IO: u64 = 32 << 10;

pub fn send(guest: &Guest, handle: u32, buf: u64, len: u64) -> u64 {
    let take = len.min(MAX_IO);
    let Some(bytes) = guest.read(buf, take as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let mut body = Vec::with_capacity(4 + bytes.len());
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&bytes);
    match call(OP_SEND, &body, 0) {
        Some((0, _)) => errno::ok(take),
        Some(_) => errno::fail(errno::EPIPE),
        None => errno::fail(errno::EIO),
    }
}

pub fn recv(guest: &Guest, handle: u32, buf: u64, len: u64) -> u64 {
    let want = len.min(MAX_IO) as usize;
    let Some((status, bytes)) = call(OP_RECV, &handle.to_le_bytes(), want) else {
        return errno::fail(errno::EIO);
    };
    if status != 0 {
        return errno::fail(errno::ECONNRESET);
    }
    if bytes.is_empty() {
        return errno::ok(0);
    }
    let n = bytes.len().min(want);
    if guest.write(buf, &bytes[..n]) < n as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(n as u64)
}

pub fn close(handle: u32) {
    let _ = call(OP_CLOSE, &handle.to_le_bytes(), 0);
}
