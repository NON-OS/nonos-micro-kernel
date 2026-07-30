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

//! The debug console byte sink.
//!
//! Every architecture brings up one UART before anything else exists, because
//! a boot with no console cannot be diagnosed. x86_64 drives the PC 16550 at
//! I/O port 0x3F8; aarch64 drives the PL011 the firmware maps for the board.
//! Only the byte-level access differs, so that is all this boundary carries:
//! the framing, locking and formatting stay in `sys::serial` above it.

/// Configure the console UART. Safe to call more than once.
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::console::init();
    #[cfg(target_arch = "aarch64")]
    pl011::init();
}

/// True once the console can accept bytes.
pub fn is_available() -> bool {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::console::is_available();
    #[cfg(target_arch = "aarch64")]
    return pl011::is_available();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return false;
}

/// Emit one byte, blocking until the transmitter has room for it.
pub fn write_byte(ch: u8) {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::console::write_byte(ch);
    #[cfg(target_arch = "aarch64")]
    pl011::write_byte(ch);
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = ch;
}

/// ARM PrimeCell PL011. `DR` takes the byte; `FR` bit 5 (`TXFF`) reports a
/// full transmit FIFO. The base address comes from the arch UART module, which
/// the boot path fills in from the device tree.
#[cfg(target_arch = "aarch64")]
mod pl011 {
    use crate::arch::aarch64::uart::uart_base;

    const DR: u64 = 0x00;
    const FR: u64 = 0x18;
    const FR_TXFF: u32 = 1 << 5;

    pub fn init() {
        // QEMU's virt board and every UEFI firmware hand the PL011 over already
        // configured, so re-programming the divisors here would only risk
        // losing the bytes the firmware is still draining.
    }

    pub fn is_available() -> bool {
        uart_base() != 0
    }

    pub fn write_byte(ch: u8) {
        let base = uart_base();
        if base == 0 {
            return;
        }
        // SAFETY: `base` is the MMIO window the boot path recorded for this
        // board's PL011; DR and FR sit inside it, and both accesses are the
        // volatile word-sized reads and writes the device expects.
        unsafe {
            while core::ptr::read_volatile((base + FR) as *const u32) & FR_TXFF != 0 {
                core::hint::spin_loop();
            }
            core::ptr::write_volatile((base + DR) as *mut u32, ch as u32);
        }
    }
}
