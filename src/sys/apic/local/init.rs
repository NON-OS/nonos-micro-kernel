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

use core::sync::atomic::Ordering;

use crate::sys::serial;

use super::constants::{
    LAPIC_ESR, LAPIC_ID, LAPIC_LVT_ERROR, LAPIC_LVT_LINT0, LAPIC_LVT_LINT1, LAPIC_LVT_TIMER,
    LAPIC_SVR, LAPIC_TPR, LAPIC_VERSION, LOCAL_APIC_DEFAULT_BASE, LVT_MASKED, SPURIOUS_VECTOR,
    SVR_APIC_ENABLE,
};
use super::regs::{lapic_read_raw, lapic_write_raw};
use super::state::{LAPIC_BASE, LAPIC_INIT};

pub fn init_local_apic() {
    if LAPIC_INIT.load(Ordering::Relaxed) {
        return;
    }
    serial::println(b"[APIC] Initializing Local APIC...");

    // Detect x2APIC hand-off before touching any register: if firmware left
    // the APIC in x2APIC mode the accessors below must use MSRs, not MMIO.
    super::x2apic::detect_mode();

    unsafe {
        // Boot phase: still on identity-mapped physical. VM init will
        // republish to the kernel-half UC virtual page via rebind.
        LAPIC_BASE.store(LOCAL_APIC_DEFAULT_BASE, Ordering::SeqCst);

        // Enable APIC and pin the spurious vector at 0xFF.
        lapic_write_raw(LAPIC_SVR, SVR_APIC_ENABLE | SPURIOUS_VECTOR);

        // Task priority 0 — do not mask any vector.
        lapic_write_raw(LAPIC_TPR, 0);

        lapic_write_raw(LAPIC_ESR, 0);
        lapic_read_raw(LAPIC_ESR);

        // Park every LVT entry until the subsystem owner installs a
        // vector through setup_timer / enable_irq.
        lapic_write_raw(LAPIC_LVT_TIMER, LVT_MASKED);
        lapic_write_raw(LAPIC_LVT_LINT0, LVT_MASKED);
        lapic_write_raw(LAPIC_LVT_LINT1, LVT_MASKED);
        lapic_write_raw(LAPIC_LVT_ERROR, LVT_MASKED);
    }

    LAPIC_INIT.store(true, Ordering::SeqCst);

    let version = unsafe { lapic_read_raw(LAPIC_VERSION) };
    let id = unsafe { lapic_read_raw(LAPIC_ID) >> 24 };
    serial::print(b"[APIC] Local APIC enabled, ID=");
    serial::print_dec(id as u64);
    serial::print(b" Version=0x");
    serial::print_hex((version & 0xFF) as u64);
    serial::println(b"");
}
