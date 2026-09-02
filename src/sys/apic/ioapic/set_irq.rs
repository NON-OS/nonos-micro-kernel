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

use super::constants::IOAPIC_REDTBL;
use super::gsi::resolve_gsi_for_legacy_irq;
use super::init::init_ioapic;
use super::regs::ioapic_write;
use super::state::{IOAPIC_GSI_BASE, IOAPIC_INIT, IOAPIC_MAX_REDIR};

// Program one IOAPIC redirection entry. `irq` is a legacy ISA IRQ
// number; we walk the MADT ISOs to find the GSI it maps to and the
// polarity/trigger bits ACPI declared. `dest` is the destination
// physical APIC ID; `flags` is the lower 32 bits of the redir entry
// minus the vector / iso polarity / trigger bits which we compose here.
pub fn ioapic_set_irq(irq: u8, vector: u8, dest: u8, flags: u32) {
    if !IOAPIC_INIT.load(Ordering::Relaxed) {
        init_ioapic();
    }

    let (gsi, iso_flags) = resolve_gsi_for_legacy_irq(irq);
    let gsi_base = IOAPIC_GSI_BASE.load(Ordering::Relaxed);
    let max_redir = IOAPIC_MAX_REDIR.load(Ordering::Relaxed) as u32;

    if gsi < gsi_base || gsi >= gsi_base + max_redir {
        serial::println(b"[APIC] ERROR: GSI outside primary IOAPIC range");
        return;
    }
    let local_pin = gsi - gsi_base;

    // Every route programmed through here is one the kernel is taking for
    // itself. Claim it before the entry is written, so a capsule binding INTx
    // is refused this line rather than reprogramming the entry underneath us.
    // Doing it at this choke point rather than at the call sites means a line
    // added later is covered without anyone remembering to.
    crate::hardware::broker::irq::reserved::reserve(gsi);

    unsafe {
        let reg_low = IOAPIC_REDTBL + local_pin * 2;
        let reg_high = reg_low + 1;
        let low = (vector as u32) | flags | iso_flags;
        let high = (dest as u32) << 24;
        // High first so the entry is fully formed when the mask bit
        // in low clears and the IOAPIC starts delivering.
        ioapic_write(reg_high, high);
        ioapic_write(reg_low, low);
    }
}
