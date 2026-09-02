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

pub(super) use super::cpu_id::cpu_id;

/// Descriptor index for an APIC id, for callers that hold an id rather than
/// being the CPU in question, such as IPI targeting and topology reporting.
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
