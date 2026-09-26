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

//! `statx`, which a current libc reaches for before it tries `stat`.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::at::resolve_at;
use super::super::resolve::key;
use super::super::store;

/// `struct statx` is 256 bytes.
const STATX: usize = 256;

/// The bits for the fields the store can answer: type, mode, size and
/// mtime. Nothing else is claimed.
const STATX_TYPE: u32 = 0x0001;
const STATX_MODE: u32 = 0x0002;
const STATX_SIZE: u32 = 0x0200;
const STATX_MTIME: u32 = 0x0020;

const S_IFDIR: u16 = 0o040_000;
const S_IFREG: u16 = 0o100_000;

pub fn statx(guest: &Guest, dirfd: u64, path: u64, out: u64) -> u64 {
    let Some(at) = resolve_at(guest, dirfd, path) else {
        return errno::fail(errno::EFAULT);
    };
    let Ok((size, is_dir, mtime, readonly)) = store::stat_full(&key(&at)) else {
        return errno::fail(errno::ENOENT);
    };
    let mode = if is_dir { S_IFDIR } else { S_IFREG } | if readonly { 0o555 } else { 0o755 };

    let mut buf = [0u8; STATX];
    buf[0..4].copy_from_slice(&(STATX_TYPE | STATX_MODE | STATX_SIZE | STATX_MTIME).to_le_bytes());
    buf[4..8].copy_from_slice(&4096u32.to_le_bytes()); // stx_blksize
    buf[28..30].copy_from_slice(&mode.to_le_bytes()); // stx_mode
    buf[40..48].copy_from_slice(&size.to_le_bytes()); // stx_size
    buf[48..56].copy_from_slice(&size.div_ceil(512).to_le_bytes()); // stx_blocks
    buf[96..104].copy_from_slice(&(mtime / 1000).to_le_bytes()); // stx_mtime.sec
    match guest.write(out, &buf) {
        n if n < 0 => errno::fail(errno::EFAULT),
        _ => errno::ok(0),
    }
}
