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
use crate::memory::paging::constants::KERNEL_ASID;
use crate::memory::paging::manager::{map_page, switch_address_space, switch_to_process_address_space};
use crate::memory::paging::types::PagePermissions;
use crate::process::core::Pid;
use crate::process::userspace::constants::{USER_STACK_BASE, USER_STACK_SIZE};
use alloc::vec::Vec;

const PAGE_SIZE: u64 = 4096;

#[derive(Debug, Clone, Copy)]
pub enum UserStackError {
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
pub fn allocate_user_stack(pid: Pid) -> Result<u64, UserStackError> {
    let top: u64 = USER_STACK_BASE;
    let size: u64 = USER_STACK_SIZE as u64;
    let bottom: u64 = top.checked_sub(size).ok_or(UserStackError::AddressSpace)?;

    if (bottom % PAGE_SIZE) != 0 || (top % PAGE_SIZE) != 0 {
        return Err(UserStackError::AddressSpace);
    }

    switch_to_process_address_space(pid).map_err(|_| UserStackError::AddressSpace)?;

    let perms = PagePermissions::READ | PagePermissions::WRITE | PagePermissions::USER;
    let pages = (size / PAGE_SIZE) as usize;
    let mut allocated: Vec<(VirtAddr, PhysAddr)> = Vec::with_capacity(pages);

    for i in 0..pages {
        let va = VirtAddr::new(bottom + (i as u64) * PAGE_SIZE);
        let frame = match frame_alloc::allocate_frame() {
            Some(f) => f,
            None => {
                rollback(&allocated);
                let _ = switch_address_space(KERNEL_ASID);
                return Err(UserStackError::FrameExhausted);
            }
        };
        unsafe {
            core::ptr::write_bytes(
                (crate::memory::layout::constants::DIRECTMAP_BASE + frame.as_u64()) as *mut u8,
                0,
                PAGE_SIZE as usize,
            );
        }
        if map_page(va, frame, perms).is_err() {
            let _ = frame_alloc::deallocate_frame(frame);
            rollback(&allocated);
            let _ = switch_address_space(KERNEL_ASID);
            return Err(UserStackError::MapFailed);
        }
        let fp = frame.as_u64();
        allocated.push((va, frame));
        #[cfg(feature = "nonos-trap-kstack-writer")]
        if pid == crate::interrupts::debug_watch::TARGET_PID
            && va.as_u64() == (crate::interrupts::debug_watch::TARGET_SLOT_VA & !0xfff)
        {
            use core::sync::atomic::Ordering as O;
            let slot_phys = fp + (crate::interrupts::debug_watch::TARGET_SLOT_VA & 0xfff);
            crate::interrupts::debug_watch::TARGET_SLOT_PHYS.store(slot_phys, O::Release);
            crate::sys::serial::print(b"[USTACK-PHYS] slot_phys=");
            crate::arch::x86_64::diag::print_hex_u64(slot_phys);
            crate::sys::serial::println(b"");
            let kss = crate::process::userspace::constants::KERNEL_STACK_SIZE as u64;
            for p in crate::process::core::PROCESS_TABLE.get_all_processes() {
                let top2 = p.kernel_stack_top.load(O::Acquire);
                if top2 == 0 {
                    continue;
                }
                let kt = top2.wrapping_sub(crate::memory::layout::constants::DIRECTMAP_BASE);
                if fp >= kt.wrapping_sub(kss) && fp < kt {
                    crate::sys::serial::print(b"[USTACK=KSTACK-LIVE] owner_pid=");
                    crate::arch::x86_64::diag::print_hex_u64(p.pid as u64);
                    crate::sys::serial::print(b" ktop=");
                    crate::arch::x86_64::diag::print_hex_u64(top2);
                    crate::sys::serial::println(b"");
                }
            }
        }
    }

    // Guard page at `bottom - PAGE_SIZE` is left unmapped on purpose.

    if switch_address_space(KERNEL_ASID).is_err() {
        // We have lost the kernel ASID; this is fatal. The caller is
        // already mid-spawn so log + halt rather than return.
        crate::sys::serial::println(b"[FATAL] user_stack: kernel ASID switch failed");
        crate::boot::halt_loop();
    }

    Ok(top - 8)
}

fn rollback(allocated: &[(VirtAddr, PhysAddr)]) {
    use crate::memory::paging::manager::unmap_page;
    for (va, frame) in allocated.iter().rev() {
        let _ = unmap_page(*va);
        let _ = frame_alloc::deallocate_frame(*frame);
    }
}
