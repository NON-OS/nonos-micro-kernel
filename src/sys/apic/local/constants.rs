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

// LAPIC physical base default; firmware can override but x86 platforms
// have used this address since the original Pentium. VM init publishes
// the kernel-half UC virtual mapping of this page via rebind_to_virt.
pub(crate) const LOCAL_APIC_DEFAULT_BASE: u64 = 0xFEE0_0000;
pub const LAPIC_PHYS_BASE: u64 = LOCAL_APIC_DEFAULT_BASE;

// Register offsets inside the LAPIC MMIO page. The set we touch:
//   ID         processor ID
//   VERSION    LAPIC version
//   TPR        task priority (left at 0 so no vectors are masked)
//   EOI        end-of-interrupt write
//   SVR        spurious vector register
//   ESR        error status
//   LVT_TIMER  local vector table entry for the timer
//   LVT_LINTx  LVT entries for LINT0/LINT1
//   LVT_ERROR  LVT entry for the error pin
//   TIMER_*    initial / current count / divide config
pub(in crate::sys::apic) const LAPIC_ID: u32 = 0x020;
pub(in crate::sys::apic) const LAPIC_VERSION: u32 = 0x030;
pub(in crate::sys::apic) const LAPIC_TPR: u32 = 0x080;
/// Processor priority. Bits 7:4 are the class currently blocking delivery: an
/// interrupt is taken only when its own class is higher than this.
pub(in crate::sys::apic) const LAPIC_PPR: u32 = 0x0A0;
pub(in crate::sys::apic) const LAPIC_EOI: u32 = 0x0B0;
/// In-service register: eight 32-bit words at 0x100, 0x110 ... 0x170, one bit
/// per vector. A bit is set from delivery until the handler writes EOI, and
/// while it is set nothing of equal or lower priority reaches this CPU again.
pub(in crate::sys::apic) const LAPIC_ISR_BASE: u32 = 0x100;
pub(in crate::sys::apic) const LAPIC_ISR_WORDS: u32 = 8;
pub(in crate::sys::apic) const LAPIC_SVR: u32 = 0x0F0;
pub(in crate::sys::apic) const LAPIC_ESR: u32 = 0x280;
pub(in crate::sys::apic) const LAPIC_LVT_TIMER: u32 = 0x320;
pub(in crate::sys::apic) const LAPIC_LVT_LINT0: u32 = 0x350;
pub(in crate::sys::apic) const LAPIC_LVT_LINT1: u32 = 0x360;
pub(in crate::sys::apic) const LAPIC_LVT_ERROR: u32 = 0x370;
pub(in crate::sys::apic) const LAPIC_TIMER_INIT: u32 = 0x380;
pub(in crate::sys::apic) const LAPIC_TIMER_CURRENT: u32 = 0x390;
pub(in crate::sys::apic) const LAPIC_TIMER_DIV: u32 = 0x3E0;
pub(in crate::sys::apic) const LAPIC_TIMER_MASKED: u32 = 1 << 16;

// SVR bit 8 (APIC SW enable) + spurious-vector slot.
pub(in crate::sys::apic) const SPURIOUS_VECTOR: u32 = 0xFF;
pub(in crate::sys::apic) const SVR_APIC_ENABLE: u32 = 1 << 8;

// LVT mask bit for any vector entry.
pub(in crate::sys::apic) const LVT_MASKED: u32 = 1 << 16;

// Timer mode bit 17 = periodic; 0 = one-shot.
pub(in crate::sys::apic) const TIMER_MODE_PERIODIC: u32 = 1 << 17;

// Kernel-internal vector for the LAPIC timer; matches the IDT slot
// installed during interrupts init.
pub const TIMER_VECTOR: u8 = 0x20;
