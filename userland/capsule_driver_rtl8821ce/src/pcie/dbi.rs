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

//! The chip's own view of its PCIe core, reached through the DBI window rather
//! than through config space. The link-power registers live here and not in the
//! config header, so a driver that only holds MMIO authority can still reach
//! them. Values are rtw88 facts (`pci.c`, `pci.h`), reimplemented not copied.

use crate::regs::Mmio;

const REG_DBI_WDATA: usize = 0x03E8;
const REG_DBI_RDATA: usize = 0x03EC;
const REG_DBI_FLAG: usize = 0x03F0;
/// Address bits 11..2 select the dword; bits 15..12 are a byte-enable mask.
const DBI_ADDR_MASK: u16 = 0x0FFC;
const DBI_WREN_SHIFT: u16 = 12;
/// Written to byte 2 of the flag register to start a write, and a read.
const DBI_WFLAG: u8 = 1;
const DBI_RFLAG: u8 = 2;
/// rtw88 retries twenty times with ten microseconds between.
const RETRIES: u32 = 20;

// Roughly ten microseconds of spinning between flag polls.
fn settle() {
    for _ in 0..2048 {
        core::hint::spin_loop();
    }
}

/// Read one byte of the PCIe core's register file. `None` if the transaction
/// never cleared its flag.
pub fn read8<M: Mmio>(mmio: &M, addr: u16) -> Option<u8> {
    mmio.write16(REG_DBI_FLAG, addr & DBI_ADDR_MASK);
    mmio.write8(REG_DBI_FLAG + 2, DBI_RFLAG);
    for _ in 0..RETRIES {
        if mmio.read8(REG_DBI_FLAG + 2) == 0 {
            return Some(mmio.read8(REG_DBI_RDATA + (addr as usize & 3)));
        }
        settle();
    }
    None
}

/// Write one byte of the PCIe core's register file. The byte within the dword is
/// selected by a write-enable bit rather than by the address alone.
pub fn write8<M: Mmio>(mmio: &M, addr: u16, data: u8) {
    let remainder = addr & 3;
    let write_addr = (addr & DBI_ADDR_MASK) | (1u16 << remainder) << DBI_WREN_SHIFT;
    mmio.write8(REG_DBI_WDATA + remainder as usize, data);
    mmio.write16(REG_DBI_FLAG, write_addr);
    mmio.write8(REG_DBI_FLAG + 2, DBI_WFLAG);
    for _ in 0..RETRIES {
        if mmio.read8(REG_DBI_FLAG + 2) == 0 {
            return;
        }
        settle();
    }
}
