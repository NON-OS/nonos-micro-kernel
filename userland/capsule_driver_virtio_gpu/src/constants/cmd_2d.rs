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
pub const VG_CMD_GET_DISPLAY_INFO: u32 = 0x0100;
pub const VG_CMD_RESOURCE_CREATE_2D: u32 = 0x0101;
pub const VG_CMD_RESOURCE_UNREF: u32 = 0x0102;
pub const VG_CMD_SET_SCANOUT: u32 = 0x0103;
pub const VG_CMD_RESOURCE_FLUSH: u32 = 0x0104;
pub const VG_CMD_TRANSFER_TO_HOST_2D: u32 = 0x0105;
pub const VG_CMD_RESOURCE_ATTACH_BACKING: u32 = 0x0106;
pub const VG_CMD_RESOURCE_DETACH_BACKING: u32 = 0x0107;
pub const VG_RESP_OK_NODATA: u32 = 0x1100;
pub const VG_RESP_OK_DISPLAY_INFO: u32 = 0x1101;
// EDID surface. Feature bit 1 unlocks GET_EDID, through which the device hands
// back the monitor's raw EDID block: the only source of the panel's physical
// size, and so of a measured DPI rather than an assumed one.
pub const VIRTIO_GPU_F_EDID: u32 = 1 << 1;
pub const VG_CMD_GET_EDID: u32 = 0x010A;
pub const VG_RESP_OK_EDID: u32 = 0x1104;
/// The device signals completion through fence_id in the response, once
/// host-side execution finished.
pub const VG_FLAG_FENCE: u32 = 1 << 0;
