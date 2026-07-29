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

//! The PC 16550 UART at I/O port 0x3F8, 115200 8N1: the x86_64 debug console.
//!
//! Probing writes a pattern to the scratch register and reads it back, so a
//! machine with no UART is detected instead of hanging on a port that never
//! reports the transmitter ready. Writes carry their own bounded retry for the
//! same reason: the console must never be able to wedge the boot.

use crate::sys::io::{inb, outb};
use core::sync::atomic::{AtomicBool, Ordering};

const PORT: u16 = 0x3F8;
const LINE_STATUS: u16 = PORT + 5;
const SCRATCH: u16 = PORT + 7;
const SCRATCH_PATTERN: u8 = 0x42;
const LSR_THR_EMPTY: u8 = 0x20;
const TX_RETRIES: u32 = 10_000;

static AVAILABLE: AtomicBool = AtomicBool::new(false);

/// Probe for the UART and, if present, configure 115200 8N1.
pub fn init() {
    // SAFETY: these are the 16550's own registers at the architectural port
    // range; the scratch round-trip proves a device is answering before any
    // configuration is written.
    unsafe {
        outb(SCRATCH, SCRATCH_PATTERN);
        if inb(SCRATCH) != SCRATCH_PATTERN {
            return;
        }
        outb(PORT + 1, 0x00); // mask interrupts while the divisor changes
        outb(PORT + 3, 0x80); // DLAB: the next two writes are the divisor
        outb(PORT + 0, 0x01); // divisor low: 115200 baud
        outb(PORT + 1, 0x00); // divisor high
        outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(PORT + 2, 0xC7); // enable and clear the FIFOs
        outb(PORT + 4, 0x0B); // RTS/DSR asserted
        AVAILABLE.store(true, Ordering::Relaxed);
    }
}

/// True once the probe in `init` has found a UART.
pub fn is_available() -> bool {
    AVAILABLE.load(Ordering::Relaxed)
}

/// Emit one byte, giving up if the transmitter never reports itself empty.
pub fn write_byte(ch: u8) {
    if !AVAILABLE.load(Ordering::Relaxed) {
        return;
    }
    // SAFETY: the UART answered the probe in `init`, so these are its live
    // registers; the retry bound keeps a wedged transmitter from hanging boot.
    unsafe {
        let mut tries = TX_RETRIES;
        while inb(LINE_STATUS) & LSR_THR_EMPTY == 0 {
            tries = tries.saturating_sub(1);
            if tries == 0 {
                return;
            }
        }
        outb(PORT, ch);
    }
}
