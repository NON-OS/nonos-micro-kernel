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

use super::types::InitOut;
use crate::constants::{
    LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS,
    STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK, STATUS_FAILED, STATUS_FEATURES_OK,
    VIRTIO_GPU_F_EDID,
};
use crate::regs::Regs;

pub fn bring_up_legacy(regs: Regs, queue_phys: u64) -> Result<InitOut, &'static str> {
    unsafe {
        regs.w8(LEG_STATUS, 0);
        regs.w8(LEG_STATUS, STATUS_ACKNOWLEDGE);
        regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER);
        let host = regs.r32(LEG_HOST_FEATURES);
        let guest = host & VIRTIO_GPU_F_EDID;
        regs.w32(LEG_GUEST_FEATURES, guest);
        regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FEATURES_OK);
        if regs.r8(LEG_STATUS) & STATUS_FEATURES_OK == 0 {
            regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
            return Err("virtio-gpu: features rejected");
        }
        regs.w16(LEG_QUEUE_SEL, 0);
        let qsize = regs.r16(LEG_QUEUE_NUM);
        if qsize == 0 {
            regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
            return Err("virtio-gpu: missing control queue");
        }
        regs.w32(LEG_QUEUE_PFN, (queue_phys >> 12) as u32);
        regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER_OK);
        Ok(InitOut {
            queue_size: qsize,
            host_features: host,
            virgl: false,
            edid: guest & VIRTIO_GPU_F_EDID != 0,
            regs,
        })
    }
}
