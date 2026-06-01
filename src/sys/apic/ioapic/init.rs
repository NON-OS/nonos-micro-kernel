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

use super::adopt::adopt_madt_ioapic_base;
use super::constants::{IOAPIC_ID, IOAPIC_REDTBL, IOAPIC_VER, REDIR_MASK_BIT};
use super::pic::disable_pic;
use super::regs::{ioapic_read, ioapic_write};
use super::state::{IOAPIC_BASE, IOAPIC_GSI_BASE, IOAPIC_INIT, IOAPIC_MAX_REDIR};

pub fn init_ioapic() {
    if IOAPIC_INIT.load(Ordering::Relaxed) {
        return;
    }

    serial::println(b"[APIC] Initializing I/O APIC...");
    disable_pic();
    adopt_madt_ioapic_base();

    unsafe {
        let ver = ioapic_read(IOAPIC_VER);
        let max_redir = ((ver >> 16) & 0xFF) as u8;
        IOAPIC_MAX_REDIR.store(max_redir.saturating_add(1), Ordering::SeqCst);

        // Mask every redirection entry; subsystems that need a vector
        // call ioapic_set_irq / enable_irq to install one.
        for i in 0..=max_redir {
            let reg_low = IOAPIC_REDTBL + (i as u32) * 2;
            ioapic_write(reg_low, REDIR_MASK_BIT);
        }
    }

    IOAPIC_INIT.store(true, Ordering::SeqCst);

    let id = unsafe { (ioapic_read(IOAPIC_ID) >> 24) & 0x0F };
    let max_redir = IOAPIC_MAX_REDIR.load(Ordering::Relaxed);
    let base = IOAPIC_BASE.load(Ordering::Relaxed);
    let gsi_base = IOAPIC_GSI_BASE.load(Ordering::Relaxed);

    serial::print(b"[APIC] I/O APIC enabled, ID=");
    serial::print_dec(id as u64);
    serial::print(b" base=0x");
    serial::print_hex(base);
    serial::print(b" gsi_base=");
    serial::print_dec(gsi_base as u64);
    serial::print(b" Max redirections=");
    serial::print_dec(max_redir as u64);
    serial::println(b"");
}
