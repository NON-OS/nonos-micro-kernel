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
pub const VIRTIO_VENDOR_ID: u16 = 0x1AF4;
pub const VIRTIO_GPU_TRANSITIONAL: u16 = 0x1010;
pub const VIRTIO_GPU_MODERN: u16 = 0x1050;
pub const BAR_OFFSET: u64 = 0;
pub const VQ_REGION_SIZE: u64 = 16384;
pub const VQ_MAX_SIZE: u16 = 256;
pub const VQ_DESC_OFFSET: usize = 0;
pub const VQ_AVAIL_OFFSET: usize = 4096;
pub const VQ_USED_OFFSET: usize = 8192;
pub const VQ_STAGING_OFFSET: usize = 12288;
pub const VQ_STAGING_LEN: usize = 4096;
pub const LEG_HOST_FEATURES: usize = 0x00;
pub const LEG_GUEST_FEATURES: usize = 0x04;
pub const LEG_QUEUE_PFN: usize = 0x08;
pub const LEG_QUEUE_NUM: usize = 0x0C;
pub const LEG_QUEUE_SEL: usize = 0x0E;
pub const LEG_QUEUE_NOTIFY: usize = 0x10;
pub const LEG_STATUS: usize = 0x12;
pub const GPU_CFG_EVENTS_READ: usize = 0x14;
pub const GPU_CFG_NUM_SCANOUTS: usize = 0x1C;
pub const GPU_CFG_NUM_CAPSETS: usize = 0x20;
pub const MOD_DEVICE_FEATURE_SELECT: usize = 0x00;
pub const MOD_DEVICE_FEATURE: usize = 0x04;
pub const MOD_DRIVER_FEATURE_SELECT: usize = 0x08;
pub const MOD_DRIVER_FEATURE: usize = 0x0C;
pub const MOD_DEVICE_STATUS: usize = 0x14;
pub const MOD_QUEUE_SELECT: usize = 0x16;
pub const MOD_QUEUE_SIZE: usize = 0x18;
pub const MOD_QUEUE_ENABLE: usize = 0x1C;
pub const MOD_QUEUE_NOTIFY_OFF: usize = 0x1E;
pub const MOD_QUEUE_DESC: usize = 0x20;
pub const MOD_QUEUE_DRIVER: usize = 0x28;
pub const MOD_QUEUE_DEVICE: usize = 0x30;
pub const FEATURE_PAGE_LOW: u32 = 0;
pub const FEATURE_PAGE_HIGH: u32 = 1;
pub const VIRTIO_F_VERSION_1_HIGH: u32 = 1;
pub const STATUS_ACKNOWLEDGE: u8 = 1;
pub const STATUS_DRIVER: u8 = 2;
pub const STATUS_DRIVER_OK: u8 = 4;
pub const STATUS_FEATURES_OK: u8 = 8;
pub const STATUS_FAILED: u8 = 128;
pub const VRING_DESC_F_NEXT: u16 = 1;
pub const VRING_DESC_F_WRITE: u16 = 2;
pub const CTRLQ_INDEX: u16 = 0;
pub const VG_CMD_GET_DISPLAY_INFO: u32 = 0x0100;
pub const VG_CMD_RESOURCE_CREATE_2D: u32 = 0x0101;
pub const VG_CMD_SET_SCANOUT: u32 = 0x0103;
pub const VG_CMD_RESOURCE_FLUSH: u32 = 0x0104;
pub const VG_CMD_TRANSFER_TO_HOST_2D: u32 = 0x0105;
pub const VG_CMD_RESOURCE_ATTACH_BACKING: u32 = 0x0106;
pub const VG_CMD_GET_EDID: u32 = 0x010A;
pub const VG_RESP_OK_NODATA: u32 = 0x1100;
pub const VG_RESP_OK_DISPLAY_INFO: u32 = 0x1101;
pub const VG_RESP_OK_EDID: u32 = 0x1104;
pub const VG_FORMAT_B8G8R8A8_UNORM: u32 = 1;
pub const VG_MAX_SCANOUTS: usize = 16;

// 3D (VirGL) surface. Feature bit 0 of the device features unlocks the 3D
// command set; the host side is virglrenderer behind a virtio-vga-gl device,
// which only exists on the modern transport. The guest never renders: it
// builds Gallium command streams and submits them for host-side execution.
pub const VIRTIO_GPU_F_VIRGL: u32 = 1 << 0;
// EDID surface. Feature bit 1 unlocks GET_EDID, through which the device hands
// back the monitor's raw EDID block: the only source of the panel's physical
// size, and so of a measured DPI rather than an assumed one.
pub const VIRTIO_GPU_F_EDID: u32 = 1 << 1;
pub const VG_CMD_GET_CAPSET_INFO: u32 = 0x0108;
pub const VG_CMD_CTX_CREATE: u32 = 0x0200;
pub const VG_CMD_SUBMIT_3D: u32 = 0x0207;
pub const VG_RESP_OK_CAPSET_INFO: u32 = 0x1102;
// Header flag: the device must signal completion of this command through the
// fence_id in the response, once host-side execution finished.
pub const VG_FLAG_FENCE: u32 = 1 << 0;
