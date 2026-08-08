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

//! The original PC mechanism: an address port and a data port.
//!
//! Only dwords move, so narrower reads shift out of the containing dword and
//! narrower writes are read-modify-write. That is a real limitation of the
//! mechanism and one reason ECAM is preferred where it exists.

mod address;
mod dword;

pub(in crate::drivers::pci::config) use address::config_address;

pub(super) fn read8(bus: u8, device: u8, function: u8, offset: u16) -> u8 {
    let shift = (offset & 3) * 8;
    ((dword::read(bus, device, function, offset) >> shift) & 0xFF) as u8
}

pub(super) fn read16(bus: u8, device: u8, function: u8, offset: u16) -> u16 {
    let shift = (offset & 2) * 8;
    ((dword::read(bus, device, function, offset) >> shift) & 0xFFFF) as u16
}

pub(super) fn read32(bus: u8, device: u8, function: u8, offset: u16) -> u32 {
    dword::read(bus, device, function, offset)
}

pub(super) fn write8(bus: u8, device: u8, function: u8, offset: u16, value: u8) {
    let shift = ((offset & 3) * 8) as u32;
    dword::modify(bus, device, function, offset, 0xFF << shift, (value as u32) << shift);
}

pub(super) fn write16(bus: u8, device: u8, function: u8, offset: u16, value: u16) {
    let shift = ((offset & 2) * 8) as u32;
    dword::modify(bus, device, function, offset, 0xFFFF << shift, (value as u32) << shift);
}

pub(super) fn write32(bus: u8, device: u8, function: u8, offset: u16, value: u32) {
    dword::write(bus, device, function, offset, value)
}
