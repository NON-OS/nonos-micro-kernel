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
use crate::constants::queue::TX_DESC_COUNT;
use crate::constants::regs::{
    DESC_EOR, DESC_FS, DESC_LS, DESC_OWN, ISR_ENABLED, ISR_TER, REG_ISR, REG_TX_POLL, TX_POLL_HPQ,
};
use crate::queue::desc::{desc, desc_mut, Descriptor};
use crate::setup::Driver;

pub fn send(driver: &mut Driver, frame: &[u8]) -> Result<(), &'static str> {
    let idx = driver.tx.cur;
    if (unsafe { desc(driver.tx.desc_va, idx) }.opts1 & DESC_OWN) != 0 {
        return Err("rtl8169 tx descriptor busy");
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            frame.as_ptr(),
            driver.tx.buffer_va(idx) as *mut u8,
            frame.len(),
        );
    }
    compiler_fence(Ordering::Release);
    let eor = if idx == TX_DESC_COUNT - 1 { DESC_EOR } else { 0 };
    let addr = driver.tx.buffer_da(idx);
    let d = Descriptor {
        opts1: DESC_OWN | DESC_FS | DESC_LS | eor | frame.len() as u32,
        opts2: 0,
        addr_lo: addr as u32,
        addr_hi: (addr >> 32) as u32,
    };
    unsafe {
        desc_mut(driver.tx.desc_va, idx, d);
        driver.regs.w8(REG_TX_POLL, TX_POLL_HPQ);
    }
    poll_done(driver, idx)?;
    let isr = unsafe { driver.regs.r16(REG_ISR) };
    if isr != 0 {
        unsafe {
            driver.regs.w16(REG_ISR, isr & ISR_ENABLED);
        }
    }
    if (isr & ISR_TER) != 0 {
        return Err("rtl8169 tx interrupt error");
    }
    driver.tx.cur = (idx + 1) % TX_DESC_COUNT;
    Ok(())
}
