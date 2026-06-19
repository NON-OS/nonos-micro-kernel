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

use nonos_libc::mk_device_release;

use super::{claim, mmio, pci, Driver};
use crate::constants::{BYTES_PER_PIXEL, CLEAR_COLOR, FB_BAR, MODE_HEIGHT, MODE_WIDTH, REG_BAR};
use crate::discover::find_bga;
use crate::dispi;
use crate::error::{BgaError, BgaResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;

pub fn run() -> BgaResult<Driver> {
    let dev = find_bga().ok_or(BgaError::DeviceNotFound)?;
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_bus_master(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let reg_map = mmio::map(dev.device_id, claim_epoch, REG_BAR, dev.reg_size)?;
    let fb_map = mmio::map(dev.device_id, claim_epoch, FB_BAR, dev.fb_size)?;
    let regs = Regs::new(reg_map.user_va);
    dispi::set_mode(regs, MODE_WIDTH, MODE_HEIGHT);
    let pixels = MODE_WIDTH as u64 * MODE_HEIGHT as u64;
    dispi::clear(fb_map.user_va, pixels, CLEAR_COLOR);
    let handles = BrokerHandles::new(dev.device_id, reg_map.grant_id, fb_map.grant_id);
    let stride = MODE_WIDTH as u32 * BYTES_PER_PIXEL;
    Ok(Driver { handles, fb_user_va: fb_map.user_va, width: MODE_WIDTH, height: MODE_HEIGHT, stride })
}
