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

use nonos_libc::mk_irq_ack;

use super::rearm::rearm;
use crate::constants::queue::{BUFFER_SIZE, RX_DESC_COUNT};
use crate::constants::regs::{
    DESC_FS, DESC_LEN_MASK, DESC_LS, DESC_OWN, ISR_ENABLED, ISR_RER, REG_ISR,
};
use crate::constants::MAX_ETHERNET_FRAME;
use crate::queue::desc::desc;
use crate::setup::Driver;

pub fn recv_one(driver: &mut Driver, out: &mut [u8]) -> Result<Option<usize>, &'static str> {
    let isr = unsafe { driver.regs.r16(REG_ISR) };
    if isr != 0 {
        unsafe {
            driver.regs.w16(REG_ISR, isr & ISR_ENABLED);
        }
    }
    if (isr & ISR_RER) != 0 {
        let _ = mk_irq_ack(driver.irq_grant);
        return Err("rtl8169 rx interrupt error");
    }
    compiler_fence(Ordering::Acquire);
    let idx = driver.rx.cur;
    let d = unsafe { desc(driver.rx.desc_va, idx) };
    if (d.opts1 & DESC_OWN) != 0 {
        let _ = mk_irq_ack(driver.irq_grant);
        return Ok(None);
    }
    let len = (d.opts1 & DESC_LEN_MASK) as usize;
    if (d.opts1 & (DESC_FS | DESC_LS)) != (DESC_FS | DESC_LS)
        || len <= 4
        || len > BUFFER_SIZE
        || len - 4 > MAX_ETHERNET_FRAME
        || len - 4 > out.len()
    {
        rearm(driver, idx);
        let _ = mk_irq_ack(driver.irq_grant);
        return Err("rtl8169 rx descriptor error");
    }
    let frame_len = len - 4;
    unsafe {
        core::ptr::copy_nonoverlapping(
            driver.rx.buffer_va(idx) as *const u8,
            out.as_mut_ptr(),
            frame_len,
        );
    }
    rearm(driver, idx);
    driver.rx.cur = (idx + 1) % RX_DESC_COUNT;
    let _ = mk_irq_ack(driver.irq_grant);
    Ok(Some(frame_len))
}
