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

use super::boot_inputs::ApBootInputs;
use crate::memory::addr::PhysAddr;
use crate::smp::constants::{AP_TRAMPOLINE_ADDR, PERCPU_STACK_SIZE};
use crate::smp::state::CPU_DESCRIPTORS;
use crate::smp::trampoline::{write_per_ap_context, PerApBootContext};
use crate::smp::{CpuDescriptor, CpuState};
use core::sync::atomic::Ordering;

const AP_START_TIMEOUT_TSC: u64 = 100_000_000;

pub(super) fn start(
    cpu_id: usize,
    apic_id: u32,
    boot: &ApBootInputs,
) -> Result<bool, &'static str> {
    let stack_base = super::stack::allocate(cpu_id)?;
    let stack_top = stack_base + PERCPU_STACK_SIZE as u64;
    let ap = &CPU_DESCRIPTORS[cpu_id];

    configure_descriptor(ap, cpu_id, apic_id, stack_base);
    write_context(cpu_id, stack_top, boot)?;
    crate::arch::x86_64::interrupt::apic::start_ap(apic_id, (AP_TRAMPOLINE_ADDR >> 12) as u8);

    if wait_online(ap) {
        crate::log_info!("[SMP] AP {} online (APIC {})", cpu_id, apic_id);
        Ok(true)
    } else {
        crate::log_error!("[SMP] AP {} (APIC {}) startup timeout", cpu_id, apic_id);
        ap.set_state(CpuState::Offline);
        Ok(false)
    }
}

fn configure_descriptor(ap: &CpuDescriptor, cpu_id: usize, apic_id: u32, stack_base: u64) {
    ap.set_identity(cpu_id as u32, apic_id, PERCPU_STACK_SIZE);
    ap.stack_base.store(stack_base, Ordering::Release);
    // Published last: the AP, and anything sending it an IPI, must see a
    // complete descriptor before they see it leave Offline.
    ap.set_state(CpuState::Starting);
}

fn write_context(cpu_id: usize, stack_top: u64, boot: &ApBootInputs) -> Result<(), &'static str> {
    let ctx = PerApBootContext::new(boot.pml4_phys, stack_top, boot.entry_ptr, cpu_id as u32);
    write_per_ap_context(PhysAddr::new(AP_TRAMPOLINE_ADDR), &ctx)
        .map_err(|_| "Failed to patch AP trampoline context")
}

fn wait_online(ap: &CpuDescriptor) -> bool {
    let start = super::time::read_tsc();
    while ap.state() != CpuState::Online {
        if super::time::read_tsc() - start > AP_START_TIMEOUT_TSC {
            return false;
        }
        core::hint::spin_loop();
    }
    true
}
