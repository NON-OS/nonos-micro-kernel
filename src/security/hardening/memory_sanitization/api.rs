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

use super::canary::init_stack_canary;
use super::erase::{dod_5220_erase, sanitize};
use super::kernel_stacks::wipe_kernel_stacks;
use super::state::{BYTES_SANITIZED, INITIALIZED, SANITIZATION_CALLS, SANITIZATION_LEVEL};
use super::types::{SanitizationLevel, SanitizationStats};
use super::user_range::wipe_user_range;
use core::sync::atomic::Ordering;

pub fn on_free(ptr: *mut u8, size: usize) {
    if INITIALIZED.load(Ordering::Relaxed) {
        sanitize(ptr, size);
    }
}

pub fn on_realloc(old_ptr: *mut u8, old_size: usize) {
    if INITIALIZED.load(Ordering::Relaxed) {
        sanitize(old_ptr, old_size);
    }
}

pub fn sanitize_process_memory(pid: u64) {
    crate::log::info!("[SANITIZE] Sanitizing memory for process {}", pid);

    // The ranges below are user virtual addresses in the target process, which
    // is not the address space this runs in. They are resolved in the owner's
    // address space and wiped through the directmap; dereferencing them here
    // would fault, or hit the caller's own pages at the same addresses.
    let Some(asid) = crate::memory::paging::manager::lookup_asid_for_process(pid as u32) else {
        return;
    };

    if let Some(pcb) = crate::process::get_process_table().find_by_pid(pid as u32) {
        let memory = pcb.memory.lock();

        wipe_user_range(asid, memory.code_start.as_u64(), memory.code_end.as_u64());

        for vma in &memory.vmas {
            wipe_user_range(asid, vma.start.as_u64(), vma.end.as_u64());
        }
    }
}

pub fn zerostate_shutdown_wipe() {
    crate::log::info!("[SANITIZE] ZeroState shutdown wipe initiated");

    let saved_level = SANITIZATION_LEVEL.load(Ordering::Relaxed);
    SANITIZATION_LEVEL.store(SanitizationLevel::Paranoid as u64, Ordering::SeqCst);

    for process in crate::process::enumerate_all_processes() {
        sanitize_process_memory(process.pid as u64);
    }

    // Kernel stacks hold what the kernel did for each process: bytes copied
    // in from user space, key material a syscall touched. They come from the
    // page allocator, so neither the heap erase nor the process wipe reaches
    // them.
    wipe_kernel_stacks();

    // Filesystem caches and the cryptofs state. Most of this is heap resident
    // and would go with the erase below, but clearing it structurally also
    // drops what the caches hold outside the heap.
    crate::fs::clear_caches();

    // The key vault walks a map that lives in the heap, so it has to run while
    // the heap is still readable.
    crate::crypto::vault::zeroize_all_keys();

    SANITIZATION_LEVEL.store(saved_level, Ordering::SeqCst);
    crate::log::info!("[SANITIZE] ZeroState shutdown wipe complete");

    // The heap goes last and nothing may allocate afterwards, because the
    // erase covers the allocator's own free list. terminate() calls into the
    // firmware from here, which does not allocate.
    //
    // The extent comes from the allocator, not from layout::KHEAP_BASE. That
    // window is only mapped by heap::init, which never runs: init_bootstrap
    // claims the heap first and init returns early once it is initialized. The
    // wipe was erasing an unmapped range while every heap-resident secret, IPC
    // payloads and loader scratch among them, stayed in DRAM.
    if let Some((heap_start, heap_size)) = crate::memory::heap::get_allocator().extent() {
        dod_5220_erase(heap_start, heap_size);
    }
}

pub fn sanitization_stats() -> SanitizationStats {
    SanitizationStats {
        bytes_sanitized: BYTES_SANITIZED.load(Ordering::Relaxed),
        sanitization_calls: SANITIZATION_CALLS.load(Ordering::Relaxed),
        level: SanitizationLevel::from_u64(SANITIZATION_LEVEL.load(Ordering::Relaxed)),
        canary_enabled: true,
    }
}

pub fn init() -> Result<(), &'static str> {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    crate::log::info!("[SECURITY] Initializing memory sanitization...");

    init_stack_canary();

    SANITIZATION_LEVEL.store(SanitizationLevel::Standard as u64, Ordering::SeqCst);

    crate::log::info!("[SECURITY] Memory sanitization initialized");
    crate::log::info!("  Level: {:?}", SanitizationLevel::Standard);
    crate::log::info!("  Stack canary: [initialized]");

    Ok(())
}

pub fn set_level(level: SanitizationLevel) {
    SANITIZATION_LEVEL.store(level as u64, Ordering::SeqCst);
    crate::log::info!("[SECURITY] Sanitization level set to {:?}", level);
}

pub fn get_level() -> SanitizationLevel {
    SanitizationLevel::from_u64(SANITIZATION_LEVEL.load(Ordering::Relaxed))
}
