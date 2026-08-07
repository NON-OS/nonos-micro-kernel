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

//! Kernel broker for runtime capsule-store persistence. Fail-closed on
//! `Capability::StoreWrite`; writes are scoped to `lba >= STORE_BASE_LBA`
//! so blockfs's 0..256 header ring is unreachable, and bounded per call.

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};

const STORE_BASE_LBA: u64 = 256;
const SECTOR: usize = 512;
const MAX_LEN: usize = 8192;

pub fn sys_store_write(lba: u64, user_ptr: u64, len: u64) -> i64 {
    if !crate::syscall::caps::current_caps_or_default().can_store_write() {
        return ERRNO_PERM;
    }
    if user_ptr == 0 || len == 0 || lba < STORE_BASE_LBA {
        return ERRNO_INVAL;
    }
    let len = len as usize;
    if len > MAX_LEN || len % SECTOR != 0 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_read(user_ptr, len).is_err() {
        return ERRNO_FAULT;
    }
    let mut buf = [0u8; MAX_LEN];
    if crate::usercopy::copy_from_user(user_ptr, &mut buf[..len]).is_err() {
        return ERRNO_FAULT;
    }
    match crate::hardware::block_device::write(lba, &buf[..len]) {
        Ok(()) => {
            crate::sys::serial::print(b"[STORE-WR] ok\n");
            len as i64
        }
        Err(_) => {
            crate::sys::serial::print(b"[STORE-WR] err\n");
            ERRNO_FAULT
        }
    }
}
