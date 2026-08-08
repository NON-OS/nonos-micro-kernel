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

use super::state::{IoApicChip, IOAPICS, ISO};
use super::types::IsoFlags;

pub(super) fn iso_flags_for(gsi: u32) -> Option<IsoFlags> {
    let cache = ISO.lock();
    cache.iso.iter().find(|e| e.gsi == gsi).map(|e| e.flags)
}

/// Translate a legacy ISA IRQ number to the Global System Interrupt it
/// lands on. Drivers name their line by ISA IRQ (keyboard = 1, mouse =
/// 12), but the IOAPIC is programmed by GSI. The MADT Interrupt Source
/// Override table remaps ISA IRQs to GSIs; when firmware declares no
/// override for this IRQ the mapping is identity. QEMU has no overrides so
/// IRQ == GSI there, but real firmware can move e.g. the timer or a legacy
/// line to a different pin, and routing the raw IRQ as a GSI then programs
/// the wrong redirection entry and the interrupt never arrives.
pub fn gsi_for_irq(bus_irq: u32) -> u32 {
    let cache = ISO.lock();
    cache.iso.iter().find(|e| e.bus_irq as u32 == bus_irq).map(|e| e.gsi).unwrap_or(bus_irq)
}

pub(super) fn locate(gsi: u32) -> Option<(IoApicChip, u32)> {
    let chips = IOAPICS.lock();
    for chip in chips.iter().flatten() {
        let end = chip.gsi_base + chip.redirs;
        if gsi >= chip.gsi_base && gsi < end {
            return Some((*chip, gsi - chip.gsi_base));
        }
    }
    None
}
