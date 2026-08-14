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
use super::region::Box3d;
use super::wire::transfer;
use crate::constants::VG_CMD_TRANSFER_FROM_HOST_3D;
use crate::device::virtqueue::ControlQueue;

/// Reading back is how the 3D path is proved rather than assumed: the guest
/// sees the pixels the host GPU actually produced.
pub fn transfer_from_host_3d(
    q: &ControlQueue,
    ctx_id: u32,
    fence_id: u64,
    resource_id: u32,
    region: &Box3d,
    offset: u64,
) -> Result<(), &'static str> {
    transfer(q, VG_CMD_TRANSFER_FROM_HOST_3D, ctx_id, fence_id, resource_id, region, offset)
}
