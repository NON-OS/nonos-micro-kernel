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

use crate::syscall::{call_raw, N_MK_CAPSULE_VERIFY};

// Mirror of the kernel CapsuleVerifyRequest wire layout
// (src/syscall/microkernel/capsule_verify/request.rs). The byte layout must
// stay identical: the kernel reads this struct verbatim from user memory.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapsuleVerifyRequest {
    pub elf_ptr: u64,
    pub cert_ptr: u64,
    pub manifest_ptr: u64,
    pub trailer_ptr: u64,
    pub elf_len: u32,
    pub cert_len: u32,
    pub manifest_len: u32,
    pub trailer_len: u32,
}

// Mirror of the kernel CapsuleVerifySummary wire layout
// (src/syscall/microkernel/capsule_verify/summary.rs), filled on success.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapsuleVerifySummary {
    pub caps: u64,
    pub tier: u8,
    pub name_len: u8,
    pub ns_len: u8,
    pub _pad: [u8; 5],
    pub name: [u8; 64],
    pub namespace: [u8; 64],
}

impl CapsuleVerifySummary {
    pub const fn zeroed() -> Self {
        Self {
            caps: 0,
            tier: 0,
            name_len: 0,
            ns_len: 0,
            _pad: [0u8; 5],
            name: [0u8; 64],
            namespace: [0u8; 64],
        }
    }
}

// Run the kernel spawn-time verification chain on four store artifacts without
// spawning. Returns 0 and fills `out` on success, or a stable negative errno:
// -13 rejected by verification, -14 fault, -22 invalid artifact or manifest.
pub fn mk_capsule_verify(req: &CapsuleVerifyRequest, out: &mut CapsuleVerifySummary) -> i64 {
    let req_ptr = req as *const CapsuleVerifyRequest as u64;
    let out_ptr = out as *mut CapsuleVerifySummary as u64;
    call_raw(N_MK_CAPSULE_VERIFY, [req_ptr, out_ptr, 0, 0, 0, 0])
}
