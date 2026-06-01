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

use super::constants::{IOAPIC_REDTBL, REDIR_MASK_BIT};
use super::gsi::resolve_gsi_for_legacy_irq;
use super::regs::{ioapic_read, ioapic_write};
use super::state::{IOAPIC_GSI_BASE, IOAPIC_INIT, IOAPIC_MAX_REDIR};

pub fn disable_irq(irq: u8) {
    if !IOAPIC_INIT.load(Ordering::Relaxed) {
        return;
    }

    let (gsi, _) = resolve_gsi_for_legacy_irq(irq);
    let gsi_base = IOAPIC_GSI_BASE.load(Ordering::Relaxed);
    let max_redir = IOAPIC_MAX_REDIR.load(Ordering::Relaxed) as u32;

    if gsi < gsi_base || gsi >= gsi_base + max_redir {
        return;
    }
    let local_pin = gsi - gsi_base;

    unsafe {
        let reg_low = IOAPIC_REDTBL + local_pin * 2;
        let current = ioapic_read(reg_low);
        ioapic_write(reg_low, current | REDIR_MASK_BIT);
    }
}
