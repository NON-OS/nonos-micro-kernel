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

use super::types::{ExecutionContext, KernelMode, ProcessContext};
use core::sync::atomic::{AtomicU64, Ordering};

const MAX_CPUS: usize = 256;

/// # Safety
/// Per-CPU context storage. Each CPU must only access its own slot.
/// Atomics ensure memory ordering across context switches.
struct PerCpuContext {
    tag: AtomicU64,
    pid: AtomicU64,
    capabilities: AtomicU64,
    page_table: AtomicU64,
}

const TAG_NONE: u64 = 0;
const TAG_KERNEL: u64 = 1;
const TAG_PROCESS: u64 = 2;

/// # Safety
/// Static array indexed by CPU ID. Each CPU writes only to its own slot.
/// Reads from other CPU slots require proper synchronization.
static CPU_CONTEXTS: [PerCpuContext; MAX_CPUS] = {
    const INIT: PerCpuContext = PerCpuContext {
        tag: AtomicU64::new(TAG_NONE),
        pid: AtomicU64::new(0),
        capabilities: AtomicU64::new(0),
        page_table: AtomicU64::new(0),
    };
    [INIT; MAX_CPUS]
};

/// # Safety
/// Reads CPU ID from GS segment base. Must be called with valid GS setup.
/// Returns value modulo MAX_CPUS to prevent out-of-bounds access.
fn cpu_id() -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        // An earlier version read the self-pointer at offset 0 and used it as
        // the CPU index, so every CPU indexed CPU_CONTEXTS by (pointer % 256),
        // a wrong and cross-CPU-aliasing slot. Read the `cpu_id` field.
        let id: u32;
        unsafe {
            core::arch::asm!(
                "mov {0:e}, gs:[{off}]",
                out(reg) id,
                off = const crate::smp::percpu::layout::CPU_ID,
                options(nostack, preserves_flags)
            );
        }
        (id as usize) % MAX_CPUS
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

/// # Safety
/// Returns current execution context for this CPU. Uses acquire ordering
/// to ensure visibility of all context data written before the tag.
pub fn get_current_context() -> ExecutionContext {
    let ctx = &CPU_CONTEXTS[cpu_id()];
    match ctx.tag.load(Ordering::Acquire) {
        TAG_KERNEL => ExecutionContext::Kernel(KernelMode::Scheduler),
        TAG_PROCESS => {
            let pid = ctx.pid.load(Ordering::Acquire) as u32;
            let caps = ctx.capabilities.load(Ordering::Acquire);
            let pt = ctx.page_table.load(Ordering::Acquire);
            ExecutionContext::Process(ProcessContext::new(pid, caps, pt))
        }
        _ => ExecutionContext::None,
    }
}

/// # Safety
/// Sets kernel execution context for this CPU. Must only be called from
/// trusted kernel code paths. Uses release ordering to ensure visibility.
pub fn set_kernel_context(mode: KernelMode) {
    let _ = mode;
    let ctx = &CPU_CONTEXTS[cpu_id()];
    ctx.tag.store(TAG_KERNEL, Ordering::Release);
}

/// # Safety
/// Sets process execution context for this CPU. Caller must ensure:
/// - pid is valid and corresponds to an active process
/// - capabilities are authorized for this process
/// - page_table points to valid page tables owned by this process
pub fn set_process_context(pid: u32, capabilities: u64, page_table: u64) {
    let ctx = &CPU_CONTEXTS[cpu_id()];
    ctx.pid.store(pid as u64, Ordering::Release);
    ctx.capabilities.store(capabilities, Ordering::Release);
    ctx.page_table.store(page_table, Ordering::Release);
    ctx.tag.store(TAG_PROCESS, Ordering::Release);
}
