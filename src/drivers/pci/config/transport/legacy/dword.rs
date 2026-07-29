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

//! The port pair itself.
//!
//! It is stateful, so the two accesses making up one operation have to be
//! atomic against each other: one CPU's address write landing between
//! another's address write and data read returns the wrong device's register.

use spin::Mutex;

use super::address::config_address;
use crate::drivers::pci::constants::{PCI_CONFIG_ADDRESS, PCI_CONFIG_DATA};
use crate::sys::io::{inl, outl};

static PORT_LOCK: Mutex<()> = Mutex::new(());

pub(super) fn read(bus: u8, device: u8, function: u8, offset: u16) -> u32 {
    let addr = config_address(bus, device, function, offset);
    let _guard = PORT_LOCK.lock();
    // SAFETY: the config port pair belongs to this module and nothing else
    // drives it. The lock makes the address/data pair atomic.
    unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        inl(PCI_CONFIG_DATA)
    }
}

pub(super) fn write(bus: u8, device: u8, function: u8, offset: u16, value: u32) {
    let addr = config_address(bus, device, function, offset);
    let _guard = PORT_LOCK.lock();
    // SAFETY: as for `read`.
    unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        outl(PCI_CONFIG_DATA, value);
    }
}

/// Replace the bits `mask` selects within the containing dword.
pub(super) fn modify(bus: u8, device: u8, function: u8, offset: u16, mask: u32, value: u32) {
    let addr = config_address(bus, device, function, offset);
    let _guard = PORT_LOCK.lock();
    // SAFETY: as for `read`. The lock also makes the read and write below one
    // indivisible modification.
    unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        let current = inl(PCI_CONFIG_DATA);
        outl(PCI_CONFIG_ADDRESS, addr);
        outl(PCI_CONFIG_DATA, (current & !mask) | (value & mask));
    }
}
