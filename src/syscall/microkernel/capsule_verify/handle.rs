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

use super::request::CapsuleVerifyRequest;
use super::summary::{fill, CapsuleVerifySummary};
use crate::syscall::microkernel::capsule_load::read_blob;
use crate::syscall::microkernel::errnos::ERRNO_FAULT;

// Run the exact spawn-time verification chain on four installer-supplied
// artifact buffers without spawning, and write a consent summary to out_ptr.
// Returns 0 on success or a negative errno; nothing is registered, mapped,
// or scheduled on any path through this call.
pub fn sys_capsule_verify(req_ptr: u64, out_ptr: u64) -> i64 {
    match run(req_ptr, out_ptr) {
        Ok(()) => 0,
        Err(errno) => errno,
    }
}

fn run(req_ptr: u64, out_ptr: u64) -> Result<(), i64> {
    let size = core::mem::size_of::<CapsuleVerifyRequest>();
    if crate::usercopy::validate_user_read(req_ptr, size).is_err() {
        return Err(ERRNO_FAULT);
    }
    let req: CapsuleVerifyRequest =
        crate::usercopy::read_user_value(req_ptr).map_err(|_| ERRNO_FAULT)?;
    let out_size = core::mem::size_of::<CapsuleVerifySummary>();
    if crate::usercopy::validate_user_write(out_ptr, out_size).is_err() {
        return Err(ERRNO_FAULT);
    }
    let elf = read_blob(req.elf_ptr, req.elf_len)?;
    let cert = read_blob(req.cert_ptr, req.cert_len)?;
    let manifest = read_blob(req.manifest_ptr, req.manifest_len)?;
    let trailer = read_trailer(req.trailer_ptr, req.trailer_len)?;
    let verified = super::verify::run(&elf, &cert, &manifest, &trailer)?;
    let summary = fill(&verified);
    crate::usercopy::write_user_value(out_ptr, &summary).map_err(|_| ERRNO_FAULT)?;
    Ok(())
}

fn read_trailer(ptr: u64, len: u32) -> Result<alloc::vec::Vec<u8>, i64> {
    if len == 0 {
        return Ok(alloc::vec::Vec::new());
    }
    read_blob(ptr, len)
}
