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

/// # Safety
///
/// Caller owns the device answering at `port` and is sending it a value its
/// register accepts. A wrong write here reprograms hardware.
#[inline]
pub unsafe fn outb(port: u16, value: u8) {
    // SAFETY: caller owns the port; `out` touches the I/O space only.
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
    }
}

/// # Safety
///
/// As [`outb`].
#[inline]
pub unsafe fn outw(port: u16, value: u16) {
    // SAFETY: caller owns the port; `out` touches the I/O space only.
    unsafe {
        core::arch::asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack));
    }
}

/// # Safety
///
/// As [`outb`].
#[inline]
pub unsafe fn outl(port: u16, value: u32) {
    // SAFETY: caller owns the port; `out` touches the I/O space only.
    unsafe {
        core::arch::asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack));
    }
}
