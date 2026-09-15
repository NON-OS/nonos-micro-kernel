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

//! The three calls this needs from vfs_pool, and nothing else.

use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::answer::Answer;

const VFS_PORT: u32 = 4104;
const MAGIC: u32 = 0x4E4F_5646;
const VERSION: u16 = 1;
pub(super) const HDR_LEN: usize = 20;

pub(super) const OP_OPEN: u16 = 1;
pub(super) const OP_CLOSE: u16 = 2;
pub(super) const OP_READ: u16 = 3;
pub(super) const OP_WRITE: u16 = 4;
pub(super) const OP_MKDIR: u16 = 8;
pub(super) const OP_STORE_PERSIST: u16 = 17;

pub(super) const O_CREATE: u32 = 1 << 0;
/// A second save must not leave the first blob's tail behind it. Both are the
/// same length today, so the bytes would be overwritten either way; the flag
/// is here because a format that grows would otherwise read back as a valid
/// blob with a corrupt tail.
pub(super) const O_TRUNC: u32 = 1 << 1;

/// Longer than the shell's 300ms: this runs when the user saves or restores a
/// wallet, not on a paint path, and the store may be busy staging packages.
const TIMEOUT_MS: u64 = 1000;

pub(super) fn call(op: u16, body: &[u8], rx: &mut [u8]) -> Answer {
    let mut tx = Vec::with_capacity(HDR_LEN + body.len());
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&1u32.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);
    let rc = mk_ipc_call_timeout(
        VFS_PORT as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if rc <= 0 || (rc as usize) < HDR_LEN + 4 {
        return Answer::Silent;
    }
    let status =
        i32::from_le_bytes([rx[HDR_LEN], rx[HDR_LEN + 1], rx[HDR_LEN + 2], rx[HDR_LEN + 3]]);
    if status != 0 {
        return Answer::Refused;
    }
    Answer::Ok(rc as usize)
}
