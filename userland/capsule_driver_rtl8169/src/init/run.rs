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

use crate::constants::regs::{
    CMD_RX_ENABLE, CMD_TX_ENABLE, ISR_ENABLED, REG_CMD, REG_IMR, REG_ISR,
};
use crate::setup::Driver;

use super::{mac, reset, rx_setup, tx_setup};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    reset::run(&driver.regs)?;
    driver.mac = mac::program(&driver.regs)?;
    rx_setup::program(&driver.regs, &driver.rx);
    tx_setup::program(&driver.regs, &driver.tx);
    unsafe {
        driver.regs.w16(REG_ISR, 0xFFFF);
        driver.regs.w16(REG_IMR, ISR_ENABLED);
        driver.regs.w8(REG_CMD, CMD_RX_ENABLE | CMD_TX_ENABLE);
    }
    Ok(())
}
