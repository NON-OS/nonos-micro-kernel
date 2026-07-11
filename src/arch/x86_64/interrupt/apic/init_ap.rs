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

//! Per-CPU LAPIC bring-up for an application processor. Every CPU has its own
//! LAPIC that comes out of INIT/SIPI software-disabled, so each AP must enable
//! and program its own registers. The mode and the MMIO mapping are global and
//! were adopted from the BSP before the SIPI was sent; only the register
//! programming happens here, per CPU.

use core::sync::atomic::Ordering;

use super::constants::*;
use super::init_x2apic::init_x2apic;
use super::mmio::mmio_w32;
use super::ops::set_tpr;
use super::state::{rdmsr, wrmsr, X2APIC_MODE};

pub unsafe fn init_ap_lapic() {
    // Hardware-enable this CPU's LAPIC (and keep it in the same mode the
    // BSP chose; the mode is a package-wide contract).
    let mut base = rdmsr(IA32_APIC_BASE) | APIC_BASE_ENABLE;
    if X2APIC_MODE.load(Ordering::Acquire) {
        base |= APIC_BASE_X2;
    }
    wrmsr(IA32_APIC_BASE, base);

    if X2APIC_MODE.load(Ordering::Acquire) {
        init_x2apic();
    } else {
        mmio_w32(LAPIC_SVR, SVR_APIC_ENABLE | VEC_SPURIOUS as u32);
        mmio_w32(LAPIC_LVT_LINT0, LVT_NMI);
        mmio_w32(LAPIC_LVT_LINT1, LVT_MASKED | LVT_LEVEL);
        mmio_w32(LAPIC_LVT_THERM, VEC_THERMAL as u32);
        mmio_w32(LAPIC_LVT_ERROR, VEC_ERROR as u32);
        mmio_w32(LAPIC_LVT_TIMER, LVT_MASKED);
    }
    set_tpr(0);
}
