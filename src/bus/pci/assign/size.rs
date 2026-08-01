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

use super::access::{read32, write32};

/// What one BAR decodes, learned the only way the spec offers: write all ones
/// and read back which bits stayed clear.
pub(super) struct Bar {
    pub size: u64,
    pub is_io: bool,
    pub is_64bit: bool,
}

/// Probe the BAR at `offset`, leaving its original value in place.
///
/// The caller must have disabled the device's decoders first: the all-ones
/// probe makes the device claim a wild address for the moment it is written,
/// and a device still decoding would answer on it.
pub(super) fn size_bar(bus: u8, device: u8, function: u8, offset: u8) -> Option<Bar> {
    let original = read32(bus, device, function, offset);
    write32(bus, device, function, offset, u32::MAX);
    let probed = read32(bus, device, function, offset);
    write32(bus, device, function, offset, original);

    if probed == 0 {
        return None;
    }

    let is_io = original & 1 != 0;
    if is_io {
        let mask = probed & !0x3;
        let size = (!(mask as u64) & 0xFFFF_FFFF).wrapping_add(1) & 0xFFFF_FFFF;
        return (size != 0).then_some(Bar { size, is_io: true, is_64bit: false });
    }

    let is_64bit = (original >> 1) & 0x3 == 0x2;
    if is_64bit {
        let high_offset = offset.checked_add(4)?;
        let high_original = read32(bus, device, function, high_offset);
        write32(bus, device, function, high_offset, u32::MAX);
        let high_probed = read32(bus, device, function, high_offset);
        write32(bus, device, function, high_offset, high_original);

        let mask = ((high_probed as u64) << 32) | ((probed & !0xF) as u64);
        let size = (!mask).wrapping_add(1);
        return (size != 0).then_some(Bar { size, is_io: false, is_64bit: true });
    }

    let mask = probed & !0xF;
    let size = (!(mask as u64) & 0xFFFF_FFFF).wrapping_add(1) & 0xFFFF_FFFF;
    (size != 0).then_some(Bar { size, is_io: false, is_64bit: false })
}
