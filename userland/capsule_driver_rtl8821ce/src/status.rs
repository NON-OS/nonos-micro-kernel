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

//! One-line bring-up status on the boot console. Real laptops have no serial
//! port, so the framebuffer console is the only place a first boot can report
//! whether the wifi chip was found, powered and answering.
//!
//! Two levels. The bring-up markers use [`line`] and always print, since a first
//! boot on new hardware needs them. The chatty per-request progress (each scan's
//! counters, each join's handshake trace) uses [`debug`], which is compiled to a
//! no-op while `DEBUG` is false, so a working desktop stays clean. The debug calls
//! are left in the code: flip `DEBUG` to bring the trace back for the next
//! hardware bring-up.

/// Whether the per-request debug trace is emitted. Off for a clean desktop; the
/// panel carries the same numbers a connected user needs.
const DEBUG: bool = false;

pub fn line(msg: &[u8]) {
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
}

/// A verbose debug line, emitted only when `DEBUG` is on.
pub fn debug(msg: &[u8]) {
    if DEBUG {
        line(msg);
    }
}

/// A verbose debug number, emitted only when `DEBUG` is on.
pub fn debug_number(n: u32) {
    if DEBUG {
        number(n);
    }
}

/// Print an unsigned decimal number on the console, for firmware versions and
/// the like. Handles zero and up to ten digits without allocating.
pub fn number(mut n: u32) {
    let mut buf = [0u8; 10];
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    line(&buf[i..]);
}

/// Print a 16-bit value as four fixed hex digits, for reporting a hardware
/// status register whose individual bits say how far a handshake progressed.
pub fn hex16(v: u16) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut buf = [0u8; 4];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = DIGITS[((v >> (12 - i * 4)) & 0xF) as usize];
    }
    line(&buf);
}
