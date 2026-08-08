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

//! Port I/O for drivers, delegated to the arch boundary that owns it.

use crate::arch::port_io;

/// # Safety
///
/// See [`crate::arch::port_io::outb`].
#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    // SAFETY: the caller carries the port-ownership obligation through.
    unsafe { port_io::outb(port, val) }
}

/// # Safety
///
/// See [`crate::arch::port_io::inb`].
#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    // SAFETY: the caller carries the port-ownership obligation through.
    unsafe { port_io::inb(port) }
}

/// # Safety
///
/// See [`crate::arch::port_io::outw`].
#[inline(always)]
pub unsafe fn outw(port: u16, val: u16) {
    // SAFETY: the caller carries the port-ownership obligation through.
    unsafe { port_io::outw(port, val) }
}

/// # Safety
///
/// See [`crate::arch::port_io::inw`].
#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    // SAFETY: the caller carries the port-ownership obligation through.
    unsafe { port_io::inw(port) }
}

/// # Safety
///
/// See [`crate::arch::port_io::outl`].
#[inline(always)]
pub unsafe fn outl(port: u16, val: u32) {
    // SAFETY: the caller carries the port-ownership obligation through.
    unsafe { port_io::outl(port, val) }
}

/// # Safety
///
/// See [`crate::arch::port_io::inl`].
#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    // SAFETY: the caller carries the port-ownership obligation through.
    unsafe { port_io::inl(port) }
}

#[inline(always)]
pub fn io_wait() {
    port_io::io_wait();
}
