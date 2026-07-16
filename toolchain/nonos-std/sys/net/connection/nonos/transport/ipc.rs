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

// The request/reply call to a net service: resolve the service port (MSVL),
// frame magic+op+body, send it with MICL under a caller-chosen deadline, map
// a lapsed deadline to TimedOut, and check the reply status. `sk`/`sk_timed`
// are the net.sockets shorthands (unbounded, and with a timeout).

use super::consts::{BODY, ERRNO_TIMEDOUT, SK_MAGIC, SK_NAME};
use super::err::err;
use super::syscall::{sys5, sys6, tag4};
use crate::io;
use crate::vec::Vec;

pub(crate) fn ipc(
    service: &[u8],
    magic: u32,
    op: u16,
    body: &[u8],
    reply_cap: usize,
    timeout_ms: u64,
) -> io::Result<Vec<u8>> {
    let mut port = 0u32;
    let mut owner = 0u32;
    let rc = unsafe {
        sys5(
            tag4(b"MSVL"),
            service.as_ptr() as u64,
            service.len() as u64,
            &mut port as *mut u32 as u64,
            &mut owner as *mut u32 as u64,
            0,
        )
    };
    if rc != 0 {
        return Err(err("net service unavailable"));
    }
    let mut tx = Vec::with_capacity(BODY + body.len());
    tx.extend_from_slice(&magic.to_le_bytes());
    tx.extend_from_slice(&1u16.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);
    let mut rx = crate::vec![0u8; BODY + reply_cap];
    let n = unsafe {
        sys6(
            tag4(b"MICL"),
            port as u64,
            tx.as_ptr() as u64,
            tx.len() as u64,
            rx.as_mut_ptr() as u64,
            rx.len() as u64,
            timeout_ms,
        )
    };
    if n == ERRNO_TIMEDOUT {
        return Err(io::Error::from(io::ErrorKind::TimedOut));
    }
    if n < BODY as i64 {
        return Err(err("net ipc failed"));
    }
    if u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        return Err(err("net op rejected"));
    }
    rx.truncate(n as usize);
    Ok(rx)
}

pub(crate) fn sk(op: u16, body: &[u8], cap: usize) -> io::Result<Vec<u8>> {
    ipc(SK_NAME, SK_MAGIC, op, body, cap, 0)
}

// Like `sk`, but with a caller-chosen deadline for the recv/send paths that
// carry a socket timeout or nonblocking mode.
pub(crate) fn sk_timed(op: u16, body: &[u8], cap: usize, timeout_ms: u64) -> io::Result<Vec<u8>> {
    ipc(SK_NAME, SK_MAGIC, op, body, cap, timeout_ms)
}
