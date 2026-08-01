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

//! Per-process user stack mapped inside the capsule's address space.
//!
//! Layout (high → low):
//!   USER_STACK_BASE                   (top, exclusive — initial RSP)
//!   [USER_STACK_BASE - 4 KiB ..= USER_STACK_BASE)   highest mapped page
//!   ...
//!   [STACK_BOTTOM .. STACK_BOTTOM + 4 KiB)          lowest mapped page
//!   [STACK_BOTTOM - 4 KiB .. STACK_BOTTOM)          GUARD: unmapped
//!
//! Guard page below the stack is intentionally unmapped so a
//! user-stack overflow page-faults as a CPL=3 user fault, not as a
//! recursive kernel fault. Pages are mapped READ | WRITE | USER and
//! never EXECUTE, so the loader's NX policy holds end-to-end.

use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::frame_alloc;
use crate::memory::paging::manager::{lookup_asid_for_process, map_page_in_asid};
use crate::memory::paging::types::PagePermissions;
use crate::process::core::Pid;
use crate::process::userspace::constants::{USER_STACK_BASE, USER_STACK_SIZE};
use alloc::vec::Vec;

const PAGE_SIZE: u64 = 4096;

#[derive(Debug, Clone, Copy)]
pub(crate) enum UserStackError {
    AddressSpace,
    FrameExhausted,
    MapFailed,
}

/// Allocate a per-process user stack in the capsule's address space.
///
/// Returns the initial RSP (top of stack, 16-byte aligned for the
/// SysV AMD64 entry ABI: a synthetic call would push an 8-byte return
/// address, so we hand back `top - 8` so a real entry sees rsp ≡ 8
/// (mod 16) before its prologue).
pub(crate) fn allocate_user_stack(pid: Pid) -> Result<u64, UserStackError> {
    let top: u64 = USER_STACK_BASE;
    let size: u64 = USER_STACK_SIZE as u64;
    let bottom: u64 = top.checked_sub(size).ok_or(UserStackError::AddressSpace)?;

    if (bottom % PAGE_SIZE) != 0 || (top % PAGE_SIZE) != 0 {
        return Err(UserStackError::AddressSpace);
    }

    let asid = lookup_asid_for_process(pid).ok_or(UserStackError::AddressSpace)?;

    let perms = PagePermissions::READ | PagePermissions::WRITE | PagePermissions::USER;
    let pages = (size / PAGE_SIZE) as usize;
    let mut allocated: Vec<(VirtAddr, PhysAddr)> = Vec::with_capacity(pages);

    for i in 0..pages {
        let va = VirtAddr::new(bottom + (i as u64) * PAGE_SIZE);
        let frame = match frame_alloc::allocate_frame() {
            Some(f) => f,
            None => {
                rollback(asid, perms, &allocated);
                return Err(UserStackError::FrameExhausted);
            }
        };
        if map_page_in_asid(asid, va, frame, perms).is_err() {
            let _ = frame_alloc::deallocate_frame(frame);
            rollback(asid, perms, &allocated);
            return Err(UserStackError::MapFailed);
        }
        allocated.push((va, frame));
    }

    // Guard page at `bottom - PAGE_SIZE` is left unmapped on purpose.

    Ok(top - 8)
}

fn rollback(asid: u32, perms: PagePermissions, allocated: &[(VirtAddr, PhysAddr)]) {
    use crate::memory::paging::manager::unmap_page_in_asid;
    for (va, frame) in allocated.iter().rev() {
        let _ = unmap_page_in_asid(asid, *va, perms);
        let _ = frame_alloc::deallocate_frame(*frame);
    }
}
