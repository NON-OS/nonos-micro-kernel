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
/// Feature bit 0 unlocks the 3D command set; the host side is virglrenderer
/// behind a virtio-vga-gl device, modern transport only. The guest never
/// renders: it builds Gallium streams for host-side execution.
pub const VIRTIO_GPU_F_VIRGL: u32 = 1 << 0;
pub const VG_CMD_GET_CAPSET_INFO: u32 = 0x0108;
pub const VG_CMD_CTX_CREATE: u32 = 0x0200;
pub const VG_CMD_CTX_ATTACH_RESOURCE: u32 = 0x0202;
pub const VG_CMD_CTX_DETACH_RESOURCE: u32 = 0x0203;
pub const VG_CMD_RESOURCE_CREATE_3D: u32 = 0x0204;
pub const VG_CMD_TRANSFER_FROM_HOST_3D: u32 = 0x0206;
pub const VG_CMD_SUBMIT_3D: u32 = 0x0207;
pub const VG_RESP_OK_CAPSET_INFO: u32 = 0x1102;

pub const VG_TARGET_TEXTURE_2D: u32 = 2;
/// The host allocates from the bind flags, so a resource that will ever be
/// scanned out must say so at creation; it cannot be widened later.
pub const VG_BIND_RENDER_TARGET: u32 = 1 << 1;
