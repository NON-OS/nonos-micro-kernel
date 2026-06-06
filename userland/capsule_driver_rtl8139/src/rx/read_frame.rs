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

use super::advance::advance;
use super::copy_ring::copy_ring;
use super::ring_u16::ring_u16;
use crate::constants::regs::RX_STATUS_OK;
use crate::constants::MAX_ETHERNET_FRAME;
use crate::setup::Driver;

pub(super) fn read_frame(
    driver: &mut Driver,
    out: &mut [u8],
) -> Result<Option<usize>, &'static str> {
    compiler_fence(Ordering::Acquire);
    let base = driver.rx_user_va;
    let off = driver.rx_offset;
    let status = ring_u16(base, off);
    let raw_len = ring_u16(base, off + 2) as usize;
    if (status & RX_STATUS_OK) == 0 || raw_len <= 4 {
        let _ = mk_irq_ack(driver.irq_grant);
        return Err("rtl8139 rx descriptor error");
    }
    let frame_len = raw_len - 4;
    if frame_len > MAX_ETHERNET_FRAME || frame_len > out.len() {
        let _ = mk_irq_ack(driver.irq_grant);
        return Err("rtl8139 rx frame too large");
    }
    copy_ring(base, off + 4, out, frame_len);
    advance(driver, raw_len)?;
    let _ = mk_irq_ack(driver.irq_grant);
    Ok(Some(frame_len))
}
