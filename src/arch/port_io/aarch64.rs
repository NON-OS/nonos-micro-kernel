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

//! A port number is an offset into the PCI bridge's I/O window.

use super::window::address_of;

macro_rules! read {
    ($name:ident, $ty:ty) => {
        /// # Safety
        ///
        /// Caller owns the device answering at `port`.
        #[inline]
        pub unsafe fn $name(port: u16) -> $ty {
            match address_of(port) {
                // SAFETY: inside the window the boot path mapped as Device.
                Some(addr) => unsafe { core::ptr::read_volatile(addr as *const $ty) },
                None => !0,
            }
        }
    };
}

macro_rules! write {
    ($name:ident, $ty:ty) => {
        /// # Safety
        ///
        /// Caller owns the device answering at `port`.
        #[inline]
        pub unsafe fn $name(port: u16, value: $ty) {
            if let Some(addr) = address_of(port) {
                // SAFETY: inside the window the boot path mapped as Device.
                unsafe {
                    core::ptr::write_volatile(addr as *mut $ty, value);
                }
            }
        }
    };
}

read!(inb, u8);
read!(inw, u16);
read!(inl, u32);

write!(outb, u8);
write!(outw, u16);
write!(outl, u32);
