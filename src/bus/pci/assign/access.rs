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

//! Config space access for the assignment pass.
//!
//! Routed through the driver layer's accessor rather than the port pair, which
//! only exists on x86_64. That accessor picks ECAM or the ports depending on
//! what the platform published, so this reads real config space on a board
//! that has no I/O ports at all.

use crate::drivers::pci::{read32_unchecked, write32_unchecked};

pub(super) fn read32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    read32_unchecked(bus, device, function, offset)
}

pub(super) fn write32(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    write32_unchecked(bus, device, function, offset, value);
}

pub(super) fn read16(bus: u8, device: u8, function: u8, offset: u8) -> u16 {
    let word = read32(bus, device, function, offset & 0xFC);
    ((word >> ((offset & 2) * 8)) & 0xFFFF) as u16
}

pub(super) fn read8(bus: u8, device: u8, function: u8, offset: u8) -> u8 {
    let word = read32(bus, device, function, offset & 0xFC);
    ((word >> ((offset & 3) * 8)) & 0xFF) as u8
}

/// Read the containing dword, replace the half we own and write it back.
///
/// Config space tolerates a narrower write, but the accessor underneath deals
/// in dwords, so this keeps the neighbouring half as it was.
pub(super) fn write16(bus: u8, device: u8, function: u8, offset: u8, value: u16) {
    let aligned = offset & 0xFC;
    let shift = (offset & 2) * 8;
    let word = read32(bus, device, function, aligned);
    let cleared = word & !(0xFFFFu32 << shift);
    write32(bus, device, function, aligned, cleared | ((value as u32) << shift));
}
