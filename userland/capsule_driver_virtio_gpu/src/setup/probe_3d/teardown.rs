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

use super::ids::{PROBE_RESOURCE_ID, RENDER_CTX_ID};
use crate::device::cmd::{ctx_detach_resource, detach_backing, resource_unref};
use crate::device::ControlQueue;
use crate::state::FenceCounter;

/// Reverse of acquisition: no context may name the resource, and no backing
/// may be attached, before the host allocation is freed. The render context
/// itself is kept: it is shared by every later 3D client.
pub fn release(q: &ControlQueue, fences: &FenceCounter) -> Result<(), &'static str> {
    ctx_detach_resource(q, RENDER_CTX_ID, PROBE_RESOURCE_ID)?;
    detach_backing(q, fences.issue(), PROBE_RESOURCE_ID)?;
    resource_unref(q, fences.issue(), PROBE_RESOURCE_ID)
}
