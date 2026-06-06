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

use nonos_libc::mk_irq_ack;

use super::read_frame::read_frame;
use crate::constants::regs::{
    CMD_RX_BUF_EMPTY, ISR_ENABLED, ISR_RX_ERR, ISR_RX_FIFO_OVERFLOW, ISR_RX_OVERFLOW, REG_CMD,
    REG_ISR,
};
use crate::setup::Driver;

pub fn recv_one(driver: &mut Driver, out: &mut [u8]) -> Result<Option<usize>, &'static str> {
    let isr = driver.pio.r16(REG_ISR)?;
    if isr != 0 {
        driver.pio.w16(REG_ISR, isr & ISR_ENABLED)?;
    }
    if (isr & (ISR_RX_ERR | ISR_RX_OVERFLOW | ISR_RX_FIFO_OVERFLOW)) != 0 {
        let _ = mk_irq_ack(driver.irq_grant);
        return Err("rtl8139 rx interrupt error");
    }
    if (driver.pio.r8(REG_CMD)? & CMD_RX_BUF_EMPTY) != 0 {
        let _ = mk_irq_ack(driver.irq_grant);
        return Ok(None);
    }
    read_frame(driver, out)
}
