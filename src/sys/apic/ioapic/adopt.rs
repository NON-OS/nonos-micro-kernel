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

use super::state::{IOAPIC_BASE, IOAPIC_GSI_BASE};

// Pull the IOAPIC MMIO base from MADT. If MADT is silent we keep the
// architectural default that init() seeded. If MADT reports multiple
// IOAPICs we drive only the one with the lowest GSI base; logs flag
// the extras so a future hardware bring-up can opt to wire them too.
pub(super) fn adopt_madt_ioapic_base() {
    let discovered = match crate::sys::apic::ioapic_madt::discover_ioapics() {
        Some(d) => d,
        None => {
            serial::println(b"[APIC] MADT IOAPIC table empty; using default 0xFEC00000");
            return;
        }
    };
    let primary = match discovered.ioapics.iter().min_by_key(|i| i.gsi_base) {
        Some(i) => i,
        None => return,
    };
    IOAPIC_BASE.store(primary.address, Ordering::SeqCst);
    IOAPIC_GSI_BASE.store(primary.gsi_base, Ordering::SeqCst);
    if discovered.ioapics.len() > 1 {
        serial::print(b"[APIC] MADT reports ");
        serial::print_dec(discovered.ioapics.len() as u64);
        serial::println(b" IOAPICs; only primary is driven");
    }
    if !discovered.isos.is_empty() {
        serial::print(b"[APIC] MADT ISOs=");
        serial::print_dec(discovered.isos.len() as u64);
        serial::println(b"");
    }
}
