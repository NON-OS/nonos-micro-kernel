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

//! Per-process kernel stack.
//!
//! Mapped in the kernel half (READ | WRITE; no USER, no EXECUTE) by
//! `memory::page_allocator::allocate_pages`. The top is stored in
//! `pcb.kernel_stack_top` and consumed by the scheduler resume hook to
//! load TSS RSP0 before iretq-ing into a CPL=3 capsule. Distinct from
//! the user stack: a CPL=3 trap must not land on the same memory the
//! capsule was using.

use crate::memory::page_allocator::{allocate_pages, PageAllocError};
use crate::process::core::{Pid, PROCESS_TABLE};
use crate::process::userspace::constants::KERNEL_STACK_SIZE;
use core::sync::atomic::Ordering;

const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub(crate) enum KernelStackError {
    NoSuchProcess,
    Allocation,
}

/// Allocate a 16 KiB kernel-only stack and stash its top on the PCB.
/// Returns the top (16-byte aligned). `allocate_pages` returns a
/// page-aligned base; `KERNEL_STACK_SIZE` is a multiple of the page
/// size, so `top - KERNEL_STACK_SIZE` reconstructs the base verbatim
/// at deallocation time without needing a second PCB field.
pub(crate) fn allocate_kernel_stack(pid: Pid) -> Result<u64, KernelStackError> {
    let pages = (KERNEL_STACK_SIZE + PAGE_SIZE - 1) / PAGE_SIZE;
    let base = allocate_pages(pages).map_err(|e| {
        crate::sys::serial::print(b"[KSTACK] alloc err: ");
        crate::sys::serial::println(page_alloc_err_name(e).as_bytes());
        KernelStackError::Allocation
    })?;
    let top = base.as_u64() + (pages as u64) * (PAGE_SIZE as u64);
    let top_aligned = top & !0xF;
    let pcb = PROCESS_TABLE.find_by_pid(pid).ok_or(KernelStackError::NoSuchProcess)?;
    pcb.kernel_stack_top.store(top_aligned, Ordering::Release);
    Ok(top_aligned)
}

fn page_alloc_err_name(e: PageAllocError) -> &'static str {
    match e {
        PageAllocError::NotInitialized => "NotInitialized",
        PageAllocError::InvalidSize => "InvalidSize",
        PageAllocError::TooManyPages => "TooManyPages",
        PageAllocError::FrameAllocationFailed => "FrameAllocationFailed",
        PageAllocError::MappingFailed => "MappingFailed",
        PageAllocError::TranslationFailed => "TranslationFailed",
        PageAllocError::UnmapFailed => "UnmapFailed",
        _ => "Unknown",
    }
}
