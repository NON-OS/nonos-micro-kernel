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
use crate::constants::VG_CMD_CTX_ATTACH_RESOURCE;
use crate::device::virtqueue::ControlQueue;

/// A command stream may only name resources its own context holds, which is
/// what keeps one capsule's context from addressing another's textures.
pub fn ctx_attach_resource(
    q: &ControlQueue,
    ctx_id: u32,
    resource_id: u32,
) -> Result<(), &'static str> {
    submit(
        q,
        VG_CMD_CTX_ATTACH_RESOURCE,
        ctx_id,
        resource_id,
        "virtio-gpu: ctx attach resource rejected",
    )
}
