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
use super::ids::{
    PROBE_CLEAR_B, PROBE_HEIGHT, PROBE_RESOURCE_ID, PROBE_SURFACE_HANDLE, PROBE_WIDTH,
    RENDER_CTX_ID,
};
use crate::constants::{VG_BIND_RENDER_TARGET, VG_FORMAT_B8G8R8A8_UNORM};
use crate::device::cmd::{resource_create_3d, submit_3d, Resource3d};
use crate::device::ControlQueue;
use crate::state::FenceCounter;
use crate::virgl::draw::{clear, ClearBuffers, Rgba};
use crate::virgl::object::create_surface;
use crate::virgl::state::{set_framebuffer, set_viewport};
use crate::virgl::stream::Stream;

pub fn create_target(q: &ControlQueue, fences: &FenceCounter) -> Result<(), &'static str> {
    let res = Resource3d::render_target(
        PROBE_RESOURCE_ID,
        VG_FORMAT_B8G8R8A8_UNORM,
        PROBE_WIDTH,
        PROBE_HEIGHT,
        VG_BIND_RENDER_TARGET,
    );
    resource_create_3d(q, fences.issue(), &res)
}

/// Bind the target and clear it. Executed by virglrenderer on the host GPU,
/// not by the guest.
pub fn clear_target(q: &ControlQueue, fences: &FenceCounter) -> Result<(), &'static str> {
    let mut s = Stream::new();
    create_surface(
        &mut s,
        PROBE_SURFACE_HANDLE,
        PROBE_RESOURCE_ID,
        VG_FORMAT_B8G8R8A8_UNORM,
    )?;
    set_framebuffer(&mut s, &[PROBE_SURFACE_HANDLE], 0)?;
    set_viewport(&mut s, PROBE_WIDTH, PROBE_HEIGHT)?;
    clear(
        &mut s,
        ClearBuffers::COLOUR0,
        Rgba { r: 0.0, g: 0.0, b: PROBE_CLEAR_B, a: 1.0 },
        1.0,
        0,
    )?;
    let stream = s.finish()?;
    submit_3d(q, RENDER_CTX_ID, fences.issue(), &stream)
}
