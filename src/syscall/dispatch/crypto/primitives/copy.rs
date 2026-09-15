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

extern crate alloc;

use crate::syscall::dispatch::errno;
use crate::syscall::SyscallResult;
use crate::usercopy::{copy_from_user, copy_to_user};
use alloc::vec::Vec;

pub(super) fn read_array<const N: usize>(ptr: u64) -> Result<[u8; N], SyscallResult> {
    if ptr == 0 {
        return Err(errno(22));
    }
    let mut out = [0u8; N];
    copy_from_user(ptr, &mut out).map_err(|_| errno(14))?;
    Ok(out)
}

pub(in crate::syscall::dispatch::crypto) fn read_vec(ptr: u64, len: u64, max: usize) -> Result<Vec<u8>, SyscallResult> {
    if ptr == 0 || len == 0 || len as usize > max {
        return Err(errno(22));
    }
    let mut out = alloc::vec![0u8; len as usize];
    copy_from_user(ptr, &mut out).map_err(|_| errno(14))?;
    Ok(out)
}

pub(in crate::syscall::dispatch::crypto) fn write(ptr: u64, bytes: &[u8]) -> SyscallResult {
    if ptr == 0 {
        return errno(22);
    }
    if copy_to_user(ptr, bytes).is_err() {
        return errno(14);
    }
    SyscallResult { value: bytes.len() as i64, capability_consumed: false, audit_required: true }
}
