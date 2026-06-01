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

use super::constants::{REDIR_ACTIVE_LOW, REDIR_LEVEL_TRIGGERED};

// Map a legacy ISA IRQ number to the GSI it lands on plus the polarity
// + trigger flags that the IOAPIC redir entry needs. ACPI's Interrupt
// Source Overrides (ISOs) are the only authority on this; if MADT did
// not declare an override the legacy identity mapping applies and the
// default edge / active-high polarity is used.
pub(super) fn resolve_gsi_for_legacy_irq(irq: u8) -> (u32, u32) {
    let discovered = match crate::sys::apic::ioapic_madt::discover_ioapics() {
        Some(d) => d,
        None => return (irq as u32, 0),
    };
    for iso in discovered.isos.iter() {
        if iso.source_irq == irq {
            let mut flags = 0u32;
            if iso.is_active_low() {
                flags |= REDIR_ACTIVE_LOW;
            }
            if iso.is_level_triggered() {
                flags |= REDIR_LEVEL_TRIGGERED;
            }
            return (iso.gsi, flags);
        }
    }
    (irq as u32, 0)
}
