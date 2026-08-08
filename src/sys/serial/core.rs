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

/*
8250/16550 UART driver for COM1 debug output.

Standard PC serial port at 0x3F8, configured for 115200 baud 8N1.
Used for kernel debug logging during boot and runtime.

Many modern machines (HP Elitedesk, some Dell Optiplex, etc) ship
without a physical serial port. The line status register reads 0xFF
on these systems, so we timeout the transmit wait to avoid hanging
the boot process. Output is simply dropped if no UART is present.
*/

use spin::Mutex;

pub const SERIAL_PORT: u16 = 0x3F8;

static SERIAL_LOCK: Mutex<()> = Mutex::new(());

/*
Run one logical output unit holding the COM1 lock with interrupts
disabled, so concurrent CPUs and same-core ISRs cannot interleave
bytes mid-line. The closure must emit only through `write_byte`;
nesting another locked print inside would self-deadlock the
non-reentrant spinlock, so callers keep the byte loop inline.
*/
pub fn with_serial_lock<R>(f: impl FnOnce() -> R) -> R {
    crate::arch::run_without_interrupts(|| {
        let _guard = SERIAL_LOCK.lock();
        f()
    })
}

/*
Initialize COM1 UART at 115200 baud. Probes for hardware presence
by checking for 0xFF on the scratch register - real UARTs won't
return all-ones. Sets SERIAL_AVAILABLE flag for fast-path skip.
*/
pub fn init() {
    crate::arch::console::init();
}

/*
Write single byte to serial port. Times out after ~10000 iterations
if the transmit buffer never becomes ready - prevents infinite hang
on machines without serial hardware.
*/
pub fn write_byte(ch: u8) {
    crate::arch::console::write_byte(ch);
}

pub fn is_available() -> bool {
    crate::arch::console::is_available()
}
