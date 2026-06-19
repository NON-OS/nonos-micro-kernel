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

use super::port::Port;
use crate::constants::ata::{ATA_READ_DMA_EXT, ATA_WRITE_DMA_EXT};
use crate::error::AhciResult;
use crate::regs::Regs;

pub fn transfer(port: &mut Port, regs: Regs, lba: u64, sectors: u32, write: bool) -> AhciResult<()> {
    let cmd = if write { ATA_WRITE_DMA_EXT } else { ATA_READ_DMA_EXT };
    super::build::build_slot0(port, cmd, lba, sectors, write);
    if let Err(e) = super::issue::issue_slot0(regs, port.base) {
        super::recover::recover(regs, port.base);
        return Err(e);
    }
    Ok(())
}
