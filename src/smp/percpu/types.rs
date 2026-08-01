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

use core::sync::atomic::{AtomicU32, AtomicU64};

/// Address-space id reserved for "no user CR3 currently active on
/// this CPU" — set at boot before any process runs, and after a CPU
/// has driven a process off without yet reloading another. The TLB
/// shootdown broadcaster uses this to compute the target CPU mask:
/// a user-VA flush against a CPU whose `active_asid == ASID_NONE`
/// does not need to reach that CPU. Kernel-VA flushes are not
/// asid-keyed; they still reach every online CPU.
pub const ASID_NONE: u32 = 0;

#[repr(C, align(4096))]
pub struct PerCpuData {
    pub self_ptr: u64,
    pub cpu_id: u32,
    pub apic_id: u32,
    pub current_process: AtomicU64,
    pub current_thread: AtomicU64,
    pub kernel_stack_top: u64,
    pub user_stack_saved: u64,
    pub syscall_scratch: [u64; 4],
    pub irq_nesting: u32,
    pub sched_lock_held: u32,
    pub random_state: AtomicU64,
    pub last_tick_tsc: AtomicU64,
    pub interrupt_disable_depth: u32,
    /// Address-space id currently executing on this CPU. Updated by
    /// `paging::manager::switch_address_space`. Read by the TLB
    /// shootdown broadcaster to filter target CPUs for a per-asid
    /// invalidation.
    pub active_asid: AtomicU32,
    /// Ticks left in the running task's slice on this CPU, and whether this
    /// CPU should reschedule at the next safe point. Both were single globals:
    /// every CPU's timer tick decremented the same counter, so N cores would
    /// have burned a slice N times faster than one, and a reschedule raised
    /// anywhere was seen everywhere. Appended after the offsets `layout.rs`
    /// asserts, so the assembly that addresses this block is unaffected.
    pub time_slice: AtomicU64,
    pub need_resched: AtomicU32,
    /// Non-zero while a TLB shootdown round has this CPU as a target. Set by
    /// the originator before it publishes the request, cleared by whichever
    /// path here does the flush first. It is what makes the flush both
    /// correctly targeted and safe to run twice.
    pub tlb_flush_pending: AtomicU32,
    _reserved: [u8; 4096 - 132],
}

impl PerCpuData {
    pub const fn new() -> Self {
        Self {
            self_ptr: 0,
            cpu_id: 0,
            apic_id: 0,
            current_process: AtomicU64::new(0),
            current_thread: AtomicU64::new(0),
            kernel_stack_top: 0,
            user_stack_saved: 0,
            syscall_scratch: [0; 4],
            irq_nesting: 0,
            sched_lock_held: 0,
            random_state: AtomicU64::new(0),
            last_tick_tsc: AtomicU64::new(0),
            interrupt_disable_depth: 0,
            active_asid: AtomicU32::new(ASID_NONE),
            time_slice: AtomicU64::new(0),
            need_resched: AtomicU32::new(0),
            tlb_flush_pending: AtomicU32::new(0),
            _reserved: [0; 4096 - 132],
        }
    }
}
