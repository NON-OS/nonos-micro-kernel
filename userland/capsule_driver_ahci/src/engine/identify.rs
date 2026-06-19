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
use crate::constants::ata::ATA_IDENTIFY;
use crate::error::AhciResult;
use crate::regs::Regs;

pub fn identify(port: &mut Port, regs: Regs) -> AhciResult<u64> {
    super::build::build_slot0(port, ATA_IDENTIFY, 0, 1, false);
    if let Err(e) = super::issue::issue_slot0(regs, port.base) {
        super::recover::recover(regs, port.base);
        return Err(e);
    }
    let words = port.data.user_va() as *const u16;
    let w = |i: usize| unsafe { core::ptr::read_volatile(words.add(i)) } as u64;
    let mut sectors = w(100) | (w(101) << 16) | (w(102) << 32) | (w(103) << 48);
    if sectors == 0 {
        sectors = w(60) | (w(61) << 16);
    }
    port.capacity_sectors = sectors;
    Ok(sectors)
}
