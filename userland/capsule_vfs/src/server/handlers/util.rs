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

use crate::protocol::{
    EACCES, EBADF, EEXIST, EINVAL, EISDIR, ENOENT, ENOSPC, ENOTEMPTY,
};
use crate::store::StoreError;

pub(super) fn map_store_err(e: StoreError) -> i32 {
    match e {
        StoreError::NotFound => ENOENT,
        StoreError::BadFd => EBADF,
        StoreError::Full => ENOSPC,
        StoreError::AccessDenied => EACCES,
        StoreError::Exists => EEXIST,
        StoreError::NotEmpty => ENOTEMPTY,
        StoreError::IsDir => EISDIR,
    }
}

// Caller pid is delivered as the first 4 bytes of every payload
// (kernel-side client embeds `state::pid()`-trusted caller pid).
// Returns `(pid, rest)` or EINVAL on a too-short payload.
pub(super) fn split_caller(payload: &[u8]) -> Result<(u32, &[u8]), i32> {
    if payload.len() < 4 {
        return Err(EINVAL);
    }
    let pid = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    Ok((pid, &payload[4..]))
}
