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

//! `MkLocalSign`: a trailer for something this machine is installing.

use crate::security::local_build::sign;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};

use super::local_image::{copy_in, within_own_authority, MAX_ELF};

/// `MkLocalSign(elf_ptr, elf_len, caps, out_ptr, out_len)`.
pub fn sys_local_sign(elf_ptr: u64, elf_len: u64, caps: u64, out_ptr: u64, out_len: u64) -> i64 {
    let elf = match copy_in(elf_ptr, elf_len, MAX_ELF) {
        Ok(bytes) => bytes,
        Err(e) => return e,
    };
    if !within_own_authority(caps) {
        return ERRNO_PERM;
    }
    let trailer = match sign(&elf, caps) {
        Ok(t) => t,
        Err(_) => return ERRNO_PERM,
    };
    let needed = trailer.len() as i64;
    if out_len == 0 {
        return needed;
    }
    if (out_len as usize) < trailer.len() {
        return ERRNO_INVAL;
    }
    match crate::usercopy::copy_to_user(out_ptr, &trailer) {
        Ok(()) => needed,
        Err(_) => ERRNO_FAULT,
    }
}
