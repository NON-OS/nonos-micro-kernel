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

// x2APIC register access for the BSP local-APIC surface. Real UEFI firmware
// frequently hands off with x2APIC already enabled, in which case the MMIO
// window is disabled and the registers are model-specific registers. We detect
// that hand-off state (without ever forcing the mode) and map an MMIO register
// offset to its MSR index: MSR = 0x800 + (offset >> 4).

use core::sync::atomic::Ordering;

use crate::arch::x86_64::boot::cpu_ops::{rdmsr, wrmsr};

use super::state::LAPIC_X2;

const IA32_APIC_BASE: u32 = 0x1B;
const APIC_BASE_X2_ENABLE: u64 = 1 << 10;
const X2APIC_MSR_BASE: u32 = 0x800;

// Latch whether firmware left the APIC in x2APIC mode. Must run before any
// register access so the accessors pick the right transport.
pub(in crate::sys::apic) fn detect_mode() {
    let enabled = unsafe { rdmsr(IA32_APIC_BASE) } & APIC_BASE_X2_ENABLE != 0;
    LAPIC_X2.store(enabled, Ordering::Release);
}

pub(in crate::sys::apic) fn is_x2(reg_order: Ordering) -> bool {
    LAPIC_X2.load(reg_order)
}

pub(in crate::sys::apic) fn read(reg: u32) -> u32 {
    unsafe { rdmsr(X2APIC_MSR_BASE + (reg >> 4)) as u32 }
}

pub(in crate::sys::apic) fn write(reg: u32, value: u32) {
    unsafe { wrmsr(X2APIC_MSR_BASE + (reg >> 4), value as u64) }
}
