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

use crate::arch::x86_64::acpi::aml::types::LpssController;

use super::interrupt::parse_extended_interrupt;
use super::memory32::parse_memory32_fixed;

const LEAD_MEMORY32_FIXED: u8 = 0x86;
const LEAD_EXTENDED_IRQ: u8 = 0x89;

/// Apply one large `_CRS` descriptor to a controller record, taking the MMIO
/// window from a Memory32Fixed descriptor and the interrupt from an Extended
/// Interrupt descriptor. Returns true once both are known so a walk can stop.
pub(super) fn apply_descriptor(lead: u8, data: &[u8], ctl: &mut LpssController) -> bool {
    match lead {
        LEAD_MEMORY32_FIXED if ctl.mmio_base == 0 => {
            if let Some((base, size)) = parse_memory32_fixed(data) {
                ctl.mmio_base = u64::from(base);
                ctl.mmio_size = size;
            }
        }
        LEAD_EXTENDED_IRQ if !ctl.has_irq => {
            if let Some(irq) = parse_extended_interrupt(data) {
                ctl.irq = irq;
                ctl.has_irq = true;
            }
        }
        _ => {}
    }
    ctl.mmio_base != 0 && ctl.has_irq
}
