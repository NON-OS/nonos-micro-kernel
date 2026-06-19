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

use super::cmd_header::CmdHeader;
use super::cmd_table::{CmdTable, PRDT_OFFSET};
use super::fis::FisH2D;
use super::port::Port;
use crate::constants::ata::{
    ATA_DEV_LBA, ATA_IDENTIFY, CMD_HEADER_WRITE, FIS_H2D_COMMAND, FIS_H2D_LEN_DWORDS,
    FIS_TYPE_REG_H2D, SECTOR_SIZE,
};

pub(super) fn build_slot0(port: &Port, cmd: u8, lba: u64, sectors: u32, write: bool) {
    let table_va = port.ctba.user_va();
    let data_phys = port.data.device_addr();
    let device = if cmd == ATA_IDENTIFY { 0 } else { ATA_DEV_LBA };
    let fis = FisH2D {
        fis_type: FIS_TYPE_REG_H2D,
        pm: FIS_H2D_COMMAND,
        command: cmd,
        featurel: 0,
        lba0: lba as u8,
        lba1: (lba >> 8) as u8,
        lba2: (lba >> 16) as u8,
        device,
        lba3: (lba >> 24) as u8,
        lba4: (lba >> 32) as u8,
        lba5: (lba >> 40) as u8,
        featureh: 0,
        countl: sectors as u8,
        counth: (sectors >> 8) as u8,
        icc: 0,
        control: 0,
        rsv: [0; 4],
    };
    let bytes = sectors as usize * SECTOR_SIZE;
    let mut flags = FIS_H2D_LEN_DWORDS;
    if write {
        flags |= CMD_HEADER_WRITE;
    }
    let header = CmdHeader {
        flags,
        pm: 0,
        prdtl: 1,
        prdbc: 0,
        ctba_low: port.ctba.device_addr() as u32,
        ctba_high: (port.ctba.device_addr() >> 32) as u32,
        rsv: [0; 4],
    };
    unsafe {
        core::ptr::write_bytes(table_va as *mut u8, 0, core::mem::size_of::<CmdTable>());
        core::ptr::write_volatile(table_va as *mut FisH2D, fis);
        super::prdt_write::prdt_write(table_va + PRDT_OFFSET as u64, data_phys, bytes as u32);
        core::ptr::write_volatile(port.clb.user_va() as *mut CmdHeader, header);
    }
}
