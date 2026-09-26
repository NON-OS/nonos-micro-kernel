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

//! `MkLocalVerify`: is this image one the machine agreed to run?

use crate::security::capsule_attest::verify_capsule_attestation;
use crate::syscall::microkernel::errnos::ERRNO_PERM;

use super::local_image::{copy_in, within_own_authority, MAX_ELF};

/// A trailer is a proof, not a payload. Anything larger is malformed.
const MAX_TRAILER: usize = 1 << 20;

/// `MkLocalVerify(elf_ptr, elf_len, caps, trailer_ptr, trailer_len)`.
pub fn sys_local_verify(
    elf_ptr: u64,
    elf_len: u64,
    caps: u64,
    trailer_ptr: u64,
    trailer_len: u64,
) -> i64 {
    let elf = match copy_in(elf_ptr, elf_len, MAX_ELF) {
        Ok(bytes) => bytes,
        Err(e) => return e,
    };
    let trailer = match copy_in(trailer_ptr, trailer_len, MAX_TRAILER) {
        Ok(bytes) => bytes,
        Err(e) => return e,
    };
    /*
     * The capabilities are bound into the proof's challenge, so asking about
     * more than the caller holds could not succeed anyway.
     */
    if !within_own_authority(caps) {
        return ERRNO_PERM;
    }
    match verify_capsule_attestation(&trailer, &elf, caps) {
        Ok(_) => 0,
        Err(_) => ERRNO_PERM,
    }
}
