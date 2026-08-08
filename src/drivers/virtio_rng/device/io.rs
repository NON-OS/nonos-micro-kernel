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

//! Register access for whichever transport the probe found this device on.
//!
//! Legacy virtio puts its registers in I/O space and the modern transport puts
//! them in a BAR. Port access goes through `sys::io`, so this file carries no
//! instruction of its own and works wherever that boundary does.

use super::core::VirtioRngDevice;
use super::types::AccessMode;
use crate::sys::io;
use core::ptr;

impl VirtioRngDevice {
    #[inline]
    pub(super) fn read8(&self, offset: u16) -> u8 {
        match &self.access {
            // SAFETY: the probe claimed this port range for this device and
            // `offset` is a virtio register inside it.
            AccessMode::Io(iobase) => unsafe { io::inb(*iobase + offset) },
            // SAFETY: the probe mapped this BAR; the offset is in range.
            AccessMode::Mmio(base) => unsafe {
                ptr::read_volatile((*base + offset as u64) as *const u8)
            },
        }
    }

    #[inline]
    pub(super) fn write8(&self, offset: u16, val: u8) {
        match &self.access {
            // SAFETY: as for `read8`.
            AccessMode::Io(iobase) => unsafe { io::outb(*iobase + offset, val) },
            // SAFETY: as for `read8`.
            AccessMode::Mmio(base) => unsafe {
                ptr::write_volatile((*base + offset as u64) as *mut u8, val);
            },
        }
    }

    #[inline]
    pub(super) fn read16(&self, offset: u16) -> u16 {
        match &self.access {
            // SAFETY: as for `read8`, two bytes wide.
            AccessMode::Io(iobase) => unsafe { io::inw(*iobase + offset) },
            // SAFETY: as for `read8`, two bytes wide.
            AccessMode::Mmio(base) => unsafe {
                ptr::read_volatile((*base + offset as u64) as *const u16)
            },
        }
    }

    #[inline]
    pub(super) fn write16(&self, offset: u16, val: u16) {
        match &self.access {
            // SAFETY: as for `read8`, two bytes wide.
            AccessMode::Io(iobase) => unsafe { io::outw(*iobase + offset, val) },
            // SAFETY: as for `read8`, two bytes wide.
            AccessMode::Mmio(base) => unsafe {
                ptr::write_volatile((*base + offset as u64) as *mut u16, val);
            },
        }
    }

    #[inline]
    pub(super) fn read32(&self, offset: u16) -> u32 {
        match &self.access {
            // SAFETY: as for `read8`, four bytes wide.
            AccessMode::Io(iobase) => unsafe { io::inl(*iobase + offset) },
            // SAFETY: as for `read8`, four bytes wide.
            AccessMode::Mmio(base) => unsafe {
                ptr::read_volatile((*base + offset as u64) as *const u32)
            },
        }
    }

    #[inline]
    pub(super) fn write32(&self, offset: u16, val: u32) {
        match &self.access {
            // SAFETY: as for `read8`, four bytes wide.
            AccessMode::Io(iobase) => unsafe { io::outl(*iobase + offset, val) },
            // SAFETY: as for `read8`, four bytes wide.
            AccessMode::Mmio(base) => unsafe {
                ptr::write_volatile((*base + offset as u64) as *mut u32, val);
            },
        }
    }
}
