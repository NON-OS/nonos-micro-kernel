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

use crate::blk::error::BlkError;
use crate::protocol::{EACCES, EBADF, EEXIST, EINVAL, EISDIR, ENOENT, ENOSPC, ENOTEMPTY};
use crate::store::StoreError;

// A name already in the TOC and the 16 MiB extent budget are both refusals a
// caller can act on, so they must not collapse into the generic EINVAL that
// every wire-level block failure maps to.
pub(super) fn map_blk_err(e: BlkError) -> i32 {
    match e {
        BlkError::Exists => EEXIST,
        BlkError::BadLength => ENOSPC,
        _ => EINVAL,
    }
}

pub(super) fn map_store_err(e: StoreError) -> i32 {
    match e {
        StoreError::NotFound => ENOENT,
        StoreError::BadFd => EBADF,
        StoreError::Full => ENOSPC,
        StoreError::AccessDenied => EACCES,
        StoreError::Exists => EEXIST,
        StoreError::NotEmpty => ENOTEMPTY,
        StoreError::IsDir => EISDIR,
        StoreError::Inval => EINVAL,
    }
}

fn resolve_caller(payload_pid: u32, sender_pid: u32) -> Option<u32> {
    if sender_pid == 0 {
        return Some(payload_pid);
    }
    if payload_pid == sender_pid {
        return Some(sender_pid);
    }
    None
}

// Caller pid is the first 4 bytes of every payload. For a CPL=3 sender
// (`sender_pid != 0`) the payload pid must match the kernel-attested
// sender pid; the kernel-side mirror (`sender_pid == 0`) is the TCB and
// keeps the payload pid. Returns `(attested_pid, rest)`, EINVAL on a
// too-short payload, or EACCES on an impersonation attempt.
pub(super) fn split_caller(payload: &[u8], sender_pid: u32) -> Result<(u32, &[u8]), i32> {
    if payload.len() < 4 {
        return Err(EINVAL);
    }
    let payload_pid = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    match resolve_caller(payload_pid, sender_pid) {
        Some(pid) => Ok((pid, &payload[4..])),
        None => Err(EACCES),
    }
}
