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

use super::wire::submit;
use crate::constants::VG_CMD_RESOURCE_UNREF;
use crate::device::virtqueue::ControlQueue;

/// Frees the host-side allocation. The resource must already be detached from
/// every context and have no backing attached.
pub fn resource_unref(
    q: &ControlQueue,
    fence_id: u64,
    resource_id: u32,
) -> Result<(), &'static str> {
    submit(
        q,
        VG_CMD_RESOURCE_UNREF,
        fence_id,
        resource_id,
        "virtio-gpu: resource_unref rejected",
    )
}
