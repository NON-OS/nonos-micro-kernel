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

use super::access::{read16, read32, write16, write32};
use super::size::size_bar;
use super::window::{alloc_io, alloc_mmio};

const COMMAND: u8 = 0x04;
const COMMAND_IO_SPACE: u16 = 1 << 0;
const COMMAND_MEMORY_SPACE: u16 = 1 << 1;
const COMMAND_BUS_MASTER: u16 = 1 << 2;
const FIRST_BAR: u8 = 0x10;
const BAR_COUNT: u8 = 6;

/// Give every unassigned BAR on one function an address.
///
/// A BAR that already holds one is left alone: firmware that assigned it may
/// have described the result to the OS by other means, and moving the window
/// afterwards would leave that description pointing at nothing.
pub(super) fn assign_device(bus: u8, device: u8, function: u8) -> bool {
    let command = read16(bus, device, function, COMMAND);
    // Sizing drives the address lines to all ones. Stop the device decoding
    // before that happens, or for the width of the probe it claims a window
    // that is not its own.
    write16(bus, device, function, COMMAND, command & !(COMMAND_IO_SPACE | COMMAND_MEMORY_SPACE));

    let mut assigned = false;
    let mut index = 0;
    while index < BAR_COUNT {
        let offset = FIRST_BAR + index * 4;
        let Some(bar) = size_bar(bus, device, function, offset) else {
            index += 1;
            continue;
        };

        let current = read32(bus, device, function, offset);
        let occupied = if bar.is_io { current & !0x3 } else { current & !0xF };
        if occupied != 0 {
            index += if bar.is_64bit { 2 } else { 1 };
            continue;
        }

        let base = if bar.is_io { alloc_io(bar.size) } else { alloc_mmio(bar.size) };
        if let Some(base) = base {
            let low = (base as u32) | (current & if bar.is_io { 0x3 } else { 0xF });
            write32(bus, device, function, offset, low);
            if bar.is_64bit {
                write32(bus, device, function, offset + 4, (base >> 32) as u32);
            }
            assigned = true;
        }

        index += if bar.is_64bit { 2 } else { 1 };
    }

    // Bus mastering goes on with the decoders: every virtio device moves its
    // queues by DMA, and without it the rings stay silent.
    write16(
        bus,
        device,
        function,
        COMMAND,
        command | COMMAND_IO_SPACE | COMMAND_MEMORY_SPACE | COMMAND_BUS_MASTER,
    );
    assigned
}
