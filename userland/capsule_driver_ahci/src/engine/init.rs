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
use super::region::DmaRegion;
use crate::constants::ata::{DATA_BUF_BYTES, STRUCT_REGION_BYTES};
use crate::constants::regs::{PORT_BASE, PORT_STRIDE};
use crate::error::AhciResult;
use crate::regs::Regs;

pub fn init_port(device_id: u64, claim_epoch: u64, regs: Regs, index: u8) -> AhciResult<Port> {
    let clb = DmaRegion::map(device_id, claim_epoch, STRUCT_REGION_BYTES)?;
    let ctba = DmaRegion::map(device_id, claim_epoch, STRUCT_REGION_BYTES)?;
    let fb = DmaRegion::map(device_id, claim_epoch, STRUCT_REGION_BYTES)?;
    let data = DmaRegion::map(device_id, claim_epoch, DATA_BUF_BYTES)?;
    let base = PORT_BASE + index as u32 * PORT_STRIDE;
    super::stop::stop(regs, base);
    super::program::program(regs, base, clb.device_addr(), fb.device_addr(), ctba.device_addr(), clb.user_va());
    super::link::link_up(regs, base)?;
    super::start::start(regs, base);
    let mut port = Port { clb, ctba, _fb: fb, data, base, capacity_sectors: 0 };
    super::identify::identify(&mut port, regs)?;
    Ok(port)
}
