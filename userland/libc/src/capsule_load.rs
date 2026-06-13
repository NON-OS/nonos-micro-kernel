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

use crate::syscall::{call_raw, N_MK_CAPSULE_LOAD};

// Mirror of the kernel CapsuleLoadRequest wire layout
// (src/syscall/microkernel/capsule_load/request.rs). The byte layout must stay
// identical: the kernel reads this struct verbatim from user memory. The four
// artifact blobs (read from the store by the caller) are passed by pointer plus
// length; the service name and endpoints come from the signed manifest, so they
// are not part of this request. u64 fields are grouped first so the layout has
// no implicit padding.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapsuleLoadRequest {
    pub elf_ptr: u64,
    pub cert_ptr: u64,
    pub manifest_ptr: u64,
    pub trailer_ptr: u64,
    pub requested_caps: u64,
    pub args_ptr: u64,
    pub elf_len: u32,
    pub cert_len: u32,
    pub manifest_len: u32,
    pub trailer_len: u32,
    pub args_len: u32,
}

// Load and spawn a capsule through the kernel verified-spawn path from artifacts
// the caller has already read from the store. Returns the new pid on success, or
// a stable negative errno: -13 rejected by verification, -14 fault, -22 invalid
// request. Gated on the IPC capability.
pub extern "C" fn mk_capsule_load(req: *const CapsuleLoadRequest) -> i64 {
    call_raw(N_MK_CAPSULE_LOAD, [req as u64, 0, 0, 0, 0, 0])
}
