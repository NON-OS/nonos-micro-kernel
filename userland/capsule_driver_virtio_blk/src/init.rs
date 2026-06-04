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
};
use super::regs::Regs;
const VIRTIO_BLK_F_FLUSH: u32 = 1 << 9;
pub struct InitOut {
    pub queue_size: u16,
}
pub fn bring_up(regs: Regs, queue_phys: u64, max_queue_size: u16) -> Result<InitOut, &'static str> {
    unsafe {
        regs.w8(LEG_STATUS, 0);
        regs.w8(LEG_STATUS, STATUS_ACKNOWLEDGE);
        let s = regs.r8(LEG_STATUS);
        regs.w8(LEG_STATUS, s | STATUS_DRIVER);
        let host = regs.r32(LEG_HOST_FEATURES);
        let want = host & VIRTIO_BLK_F_FLUSH;
        regs.w32(LEG_GUEST_FEATURES, want);
        let s2 = regs.r8(LEG_STATUS);
        regs.w8(LEG_STATUS, s2 | STATUS_FEATURES_OK);
        let s3 = regs.r8(LEG_STATUS);
        if s3 & STATUS_FEATURES_OK == 0 {
            regs.w8(LEG_STATUS, s3 | STATUS_FAILED);
            return Err("virtio-blk: features-ok rejected");
        }
        regs.w16(LEG_QUEUE_SEL, 0);
        let qmax = regs.r16(LEG_QUEUE_NUM);
        if qmax == 0 {
            regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
            return Err("virtio-blk: requestq missing");
        }
        if qmax < 3 || qmax > max_queue_size {
            regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
            return Err("virtio-blk: unsupported requestq size");
        }
        let pfn = (queue_phys >> 12) as u32;
        regs.w32(LEG_QUEUE_PFN, pfn);
        regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER_OK);
        Ok(InitOut { queue_size: qmax })
    }
}
