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
use nonos_libc::Deadline;

use crate::regs::{mmio_read32, mmio_write32};

// USBLEGSUP ownership semaphore bits (xHCI 1.2 §7.1.1).
const USBLEGSUP_BIOS_OWNED: u32 = 1 << 16;
const USBLEGSUP_OS_OWNED: u32 = 1 << 24;
// USBLEGCTLSTS (offset +4): mask of bits to preserve when clearing every SMI
// enable, and the RW1C SMI status bits to acknowledge. Same values Linux uses.
const CTLSTS_DISABLE_SMI: u32 = (0x7 << 1) | (0xff << 5) | (0x7 << 17);
const CTLSTS_SMI_EVENTS: u32 = 0x7 << 29;

const HANDOFF_TIMEOUT_MS: u64 = 1_000;

pub(super) fn claim(usblegsup: u64) {
    // Request OS ownership, then wait for BIOS to drop its semaphore.
    mmio_write32(usblegsup, mmio_read32(usblegsup) | USBLEGSUP_OS_OWNED);
    let mut owned = false;
    let deadline = Deadline::after_ms(HANDOFF_TIMEOUT_MS);
    loop {
        let v = mmio_read32(usblegsup);
        if v & USBLEGSUP_BIOS_OWNED == 0 && v & USBLEGSUP_OS_OWNED != 0 {
            owned = true;
            break;
        }
        if deadline.expired() {
            break;
        }
        core::hint::spin_loop();
    }
    // Buggy firmware that never releases: force the BIOS-owned bit down so the
    // controller is not left shared.
    if !owned {
        mmio_write32(usblegsup, mmio_read32(usblegsup) & !USBLEGSUP_BIOS_OWNED);
    }
    // Disable every SMI source and acknowledge pending SMI status so SMM stops
    // trapping on this controller once we own it.
    let ctlsts = usblegsup + 4;
    let v = mmio_read32(ctlsts);
    mmio_write32(ctlsts, (v & CTLSTS_DISABLE_SMI) | CTLSTS_SMI_EVENTS);
}
