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

//! The dword primitives go through the driver layer's accessor, which picks
//! ECAM or the 0xCF8 port pair from what the platform published. The ports are
//! an x86_64 mechanism, and on a board that reaches config space only as
//! memory, going straight to them left this layer, and every driver that
//! enumerates through it, finding nothing at all.

pub fn pci_read32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    crate::drivers::pci::read32_unchecked(bus, device, function, offset)
}

pub fn pci_read16(bus: u8, device: u8, function: u8, offset: u8) -> u16 {
    let value = pci_read32(bus, device, function, offset & 0xFC);
    ((value >> ((offset & 2) * 8)) & 0xFFFF) as u16
}

pub fn pci_read8(bus: u8, device: u8, function: u8, offset: u8) -> u8 {
    let value = pci_read32(bus, device, function, offset & 0xFC);
    ((value >> ((offset & 3) * 8)) & 0xFF) as u8
}

pub fn pci_write32(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    crate::drivers::pci::write32_unchecked(bus, device, function, offset, value);
}

pub fn pci_write16(bus: u8, device: u8, function: u8, offset: u8, value: u16) {
    let old = pci_read32(bus, device, function, offset & 0xFC);
    let shift = (offset & 2) * 8;
    let new_value = (old & !(0xFFFFu32 << shift)) | ((value as u32) << shift);
    pci_write32(bus, device, function, offset & 0xFC, new_value);
}

pub fn pci_write8(bus: u8, device: u8, function: u8, offset: u8, value: u8) {
    let old = pci_read32(bus, device, function, offset & 0xFC);
    let shift = (offset & 3) * 8;
    let new_value = (old & !(0xFFu32 << shift)) | ((value as u32) << shift);
    pci_write32(bus, device, function, offset & 0xFC, new_value);
}
