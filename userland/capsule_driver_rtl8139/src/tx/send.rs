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

use core::sync::atomic::{compiler_fence, Ordering};

use super::poll_done::poll_done;
use crate::constants::dma::{TX_SLOT_BYTES, TX_SLOT_COUNT};
use crate::constants::regs::REG_TXSTATUS0;
use crate::setup::Driver;

pub fn send(driver: &mut Driver, frame: &[u8]) -> Result<(), &'static str> {
    let idx = driver.tx_cur;
    let va = driver.tx_user_va + (idx * TX_SLOT_BYTES) as u64;
    unsafe {
        core::ptr::copy_nonoverlapping(frame.as_ptr(), va as *mut u8, frame.len());
    }
    compiler_fence(Ordering::Release);
    let status_reg = REG_TXSTATUS0 + (idx as u16 * 4);
    driver.pio.w32(status_reg, frame.len() as u32)?;
    poll_done(driver, status_reg)?;
    driver.tx_cur = (idx + 1) % TX_SLOT_COUNT;
    Ok(())
}
