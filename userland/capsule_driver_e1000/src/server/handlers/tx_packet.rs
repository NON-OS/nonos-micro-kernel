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

use crate::constants::queue::TX_DESC_COUNT;
use crate::constants::regs::REG_TDT;
use crate::constants::{MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
use crate::protocol::{Request, E_INVAL, E_IO, E_MSGSIZE, MAX_TX_PAYLOAD_BYTES};
use crate::server::error::reply_with_status;
use crate::setup::Driver;

const TX_DD_POLL_BUDGET: u32 = 1_000_000;

pub fn handle(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    if req.payload_len as usize != body.len() {
        reply_with_status(tx, req, E_MSGSIZE);
        return;
    }
    if body.len() < MIN_ETHERNET_FRAME
        || body.len() > MAX_ETHERNET_FRAME
        || body.len() as u32 > MAX_TX_PAYLOAD_BYTES
    {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let dst = driver.tx.buffer_va(driver.tx.tail) as *mut u8;
    unsafe {
        core::ptr::copy_nonoverlapping(body.as_ptr(), dst, body.len());
    }
    let idx = driver.tx.post(body.len() as u16);
    let next_tdt = ((idx as u32) + 1) % (TX_DESC_COUNT as u32);
    unsafe {
        driver.regs.w32(REG_TDT, next_tdt);
    }
    let mut spins = 0u32;
    while !driver.tx.done(idx) {
        spins += 1;
        if spins > TX_DD_POLL_BUDGET {
            reply_with_status(tx, req, E_IO);
            return;
        }
        core::hint::spin_loop();
    }
    reply_with_status(tx, req, 0);
}
