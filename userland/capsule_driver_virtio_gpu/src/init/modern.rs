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
    CTRLQ_INDEX, FEATURE_PAGE_HIGH, FEATURE_PAGE_LOW, MOD_DEVICE_FEATURE,
    MOD_DEVICE_FEATURE_SELECT, MOD_DEVICE_STATUS, MOD_DRIVER_FEATURE, MOD_DRIVER_FEATURE_SELECT,
    MOD_QUEUE_DESC, MOD_QUEUE_DEVICE, MOD_QUEUE_DRIVER, MOD_QUEUE_ENABLE, MOD_QUEUE_NOTIFY_OFF,
    MOD_QUEUE_SELECT, MOD_QUEUE_SIZE, STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK,
    STATUS_FAILED, STATUS_FEATURES_OK, VIRTIO_F_VERSION_1_HIGH, VIRTIO_GPU_F_EDID, VQ_AVAIL_OFFSET,
    VQ_DESC_OFFSET, VQ_MAX_SIZE, VQ_USED_OFFSET,
};
use crate::regs::Regs;

pub fn bring_up_modern(regs: Regs, queue_phys: u64) -> Result<InitOut, &'static str> {
    unsafe {
        regs.w8(MOD_DEVICE_STATUS, 0);
        regs.w8(MOD_DEVICE_STATUS, STATUS_ACKNOWLEDGE);
        regs.w8(MOD_DEVICE_STATUS, regs.r8(MOD_DEVICE_STATUS) | STATUS_DRIVER);
        regs.w32(MOD_DEVICE_FEATURE_SELECT, FEATURE_PAGE_LOW);
        let host = regs.r32(MOD_DEVICE_FEATURE);
        regs.w32(MOD_DEVICE_FEATURE_SELECT, FEATURE_PAGE_HIGH);
        if regs.r32(MOD_DEVICE_FEATURE) & VIRTIO_F_VERSION_1_HIGH == 0 {
            regs.w8(MOD_DEVICE_STATUS, regs.r8(MOD_DEVICE_STATUS) | STATUS_FAILED);
            return Err("virtio-gpu: modern feature missing");
        }
        // Accept the 3D command set when the host offers it (virtio-vga-gl
        // with a virglrenderer backend) and EDID when it is on offer;
        // everything else in the low page is declined as before.
        regs.w32(MOD_DRIVER_FEATURE_SELECT, FEATURE_PAGE_LOW);
        let low = host & (crate::constants::VIRTIO_GPU_F_VIRGL | VIRTIO_GPU_F_EDID);
        regs.w32(MOD_DRIVER_FEATURE, low);
        regs.w32(MOD_DRIVER_FEATURE_SELECT, FEATURE_PAGE_HIGH);
        regs.w32(MOD_DRIVER_FEATURE, VIRTIO_F_VERSION_1_HIGH);
        regs.w8(MOD_DEVICE_STATUS, regs.r8(MOD_DEVICE_STATUS) | STATUS_FEATURES_OK);
        if regs.r8(MOD_DEVICE_STATUS) & STATUS_FEATURES_OK == 0 {
            regs.w8(MOD_DEVICE_STATUS, regs.r8(MOD_DEVICE_STATUS) | STATUS_FAILED);
            return Err("virtio-gpu: features rejected");
        }
        regs.w16(MOD_QUEUE_SELECT, CTRLQ_INDEX);
        let qmax = regs.r16(MOD_QUEUE_SIZE);
        if qmax == 0 {
            regs.w8(MOD_DEVICE_STATUS, regs.r8(MOD_DEVICE_STATUS) | STATUS_FAILED);
            return Err("virtio-gpu: missing control queue");
        }
        let qsize = core::cmp::min(qmax, VQ_MAX_SIZE);
        regs.w16(MOD_QUEUE_SIZE, qsize);
        let regs = regs.with_queue_notify(regs.r16(MOD_QUEUE_NOTIFY_OFF));
        regs.w64(MOD_QUEUE_DESC, queue_phys + VQ_DESC_OFFSET as u64);
        regs.w64(MOD_QUEUE_DRIVER, queue_phys + VQ_AVAIL_OFFSET as u64);
        regs.w64(MOD_QUEUE_DEVICE, queue_phys + VQ_USED_OFFSET as u64);
        regs.w16(MOD_QUEUE_ENABLE, 1);
        regs.w8(MOD_DEVICE_STATUS, regs.r8(MOD_DEVICE_STATUS) | STATUS_DRIVER_OK);
        Ok(InitOut {
            queue_size: qsize,
            host_features: host,
            virgl: host & crate::constants::VIRTIO_GPU_F_VIRGL != 0,
            edid: low & VIRTIO_GPU_F_EDID != 0,
            regs,
        })
    }
}
