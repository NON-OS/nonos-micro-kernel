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

use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

const MAX_CPUS: usize = 256;

/// # Safety
/// Per-CPU interrupt context state. Tracks if we're in interrupt handler.
static IN_INTERRUPT: [AtomicBool; MAX_CPUS] = {
    const INIT: AtomicBool = AtomicBool::new(false);
    [INIT; MAX_CPUS]
};

/// # Safety
/// Interrupt nesting depth per CPU.
static INTERRUPT_DEPTH: [AtomicU8; MAX_CPUS] = {
    const INIT: AtomicU8 = AtomicU8::new(0);
    [INIT; MAX_CPUS]
};

/// # Safety
/// Reads the CPU id from the per-CPU data block pointed to by GS base.
/// The `cpu_id` field sits at offset 8 of `PerCpuData` (after the
/// `self_ptr: u64` at offset 0); `gs:0` is the self-pointer, which is
/// 4096-aligned and therefore useless as an index. Callers must run
/// with the kernel GS base loaded (post-swapgs).
fn cpu_id() -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        let id: u32;
        unsafe {
            core::arch::asm!(
                "mov {0:e}, gs:[{off}]",
                out(reg) id,
                off = const crate::smp::percpu::layout::CPU_ID,
                options(nostack, preserves_flags),
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
/// RAII guard for interrupt context. Sets context on creation, clears on drop.
pub struct InterruptContext {
    cpu: usize,
    _prev_depth: u8,
}

impl Drop for InterruptContext {
    fn drop(&mut self) {
        let depth = INTERRUPT_DEPTH[self.cpu].fetch_sub(1, Ordering::Release);
        if depth == 1 {
            IN_INTERRUPT[self.cpu].store(false, Ordering::Release);
        }
    }
}

/// # Safety
/// Sets interrupt context for current CPU. Must be called at interrupt entry.
pub fn set_interrupt_context() -> InterruptContext {
    let cpu = cpu_id();
    let prev_depth = INTERRUPT_DEPTH[cpu].fetch_add(1, Ordering::Acquire);
    IN_INTERRUPT[cpu].store(true, Ordering::Release);
    InterruptContext { cpu, _prev_depth: prev_depth }
}

/// # Safety
/// Returns true if current CPU is in interrupt context.
pub fn in_interrupt_context() -> bool {
    IN_INTERRUPT[cpu_id()].load(Ordering::Acquire)
}
