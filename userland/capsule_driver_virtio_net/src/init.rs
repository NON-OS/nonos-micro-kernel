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

use super::constants::{
    LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS,
    STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK, STATUS_FAILED, STATUS_FEATURES_OK,
    VIRTIO_NET_F_MAC, VIRTIO_NET_F_STATUS,
};
use super::regs::Regs;

const fn bit(n: u32) -> u32 {
    1u32 << n
}

pub fn negotiate(regs: Regs) -> Result<u32, &'static str> {
    unsafe {
        regs.w8(LEG_STATUS, 0);
        regs.w8(LEG_STATUS, STATUS_ACKNOWLEDGE);
        regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER);

        let host = regs.r32(LEG_HOST_FEATURES);
        let want = host & (bit(VIRTIO_NET_F_MAC) | bit(VIRTIO_NET_F_STATUS));
        regs.w32(LEG_GUEST_FEATURES, want);

        let s = regs.r8(LEG_STATUS);
        regs.w8(LEG_STATUS, s | STATUS_FEATURES_OK);
        let s2 = regs.r8(LEG_STATUS);
        if s2 & STATUS_FEATURES_OK == 0 {
            regs.w8(LEG_STATUS, s2 | STATUS_FAILED);
            return Err("virtio-net: features-ok rejected");
        }
        Ok(want)
    }
}

pub fn program_queue(
    regs: Regs,
    queue_index: u16,
    queue_phys: u64,
    queue_size_hint: u16,
) -> Result<u16, &'static str> {
    unsafe {
        regs.w16(LEG_QUEUE_SEL, queue_index);
        let qmax = regs.r16(LEG_QUEUE_NUM);
        if qmax == 0 {
            regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
            return Err("virtio-net: queue missing");
        }
        let qsize = core::cmp::min(qmax, queue_size_hint);
        let pfn = (queue_phys >> 12) as u32;
        regs.w32(LEG_QUEUE_PFN, pfn);
        Ok(qsize)
    }
}

pub fn driver_ok(regs: Regs) {
    unsafe {
        regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER_OK);
    }
}
