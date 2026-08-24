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

use super::state::{BSP_APIC_ID, CPU_COUNT, CPU_DESCRIPTORS};
use super::types::CpuDescriptor;
use core::sync::atomic::Ordering;

/// Which CPU is running this code.
///
/// Read from this CPU's own per-CPU block, not searched for. The previous
/// version scanned the descriptor table for a matching APIC id and fell back
/// to 0 when it found none, which is the one answer that must never be
/// guessed: the current process is tracked per CPU and every capability check
/// in the syscall layer is keyed on it, so a second CPU quietly claiming to
/// be CPU 0 would read and write another CPU's current process and be granted
/// its authority.
#[inline]
pub fn cpu_id() -> usize {
    if !crate::arch::percpu_ready() {
        // Only the boot CPU runs before its block is installed, and it is 0.
        return 0;
    }
    // SAFETY: ek@nonos.systems - the check above is exactly this call's
    // precondition: the per-CPU register has been pointed at a block.
    unsafe { crate::arch::percpu_cpu_id() as usize }
}

/// Descriptor index for an APIC id, for callers that hold an id rather than
/// being the CPU in question — IPI targeting and topology reporting. Linear
/// over the online CPUs, and `None` when the id is not one of them.
pub fn apic_to_cpu_id(apic_id: u32) -> Option<usize> {
    (0..CPU_COUNT.load(Ordering::Acquire)).find(|&i| CPU_DESCRIPTORS[i].get_apic_id() == apic_id)
}

#[inline]
pub fn current_cpu() -> &'static CpuDescriptor {
    &CPU_DESCRIPTORS[cpu_id()]
}

pub fn get_cpu(id: usize) -> Option<&'static CpuDescriptor> {
    if id < CPU_COUNT.load(Ordering::Acquire) {
        Some(&CPU_DESCRIPTORS[id])
    } else {
        None
    }
}

#[inline]
pub fn is_bsp() -> bool {
    crate::arch::cpu::get_cpu_id() == BSP_APIC_ID.load(Ordering::Acquire)
}
