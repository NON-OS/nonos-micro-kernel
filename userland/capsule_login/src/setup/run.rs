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

use crate::clients::compositor;
use crate::render;
use crate::state::Context;

pub fn run() -> Result<Context, &'static str> {
    let keyring_port = super::discover::lookup_keyring_port()?;
    let desktop_shell_port = super::discover::lookup_desktop_shell_port()?;
    let compositor_port = super::discover::lookup_compositor_port()?;
    compositor::healthcheck(compositor_port, 1).map_err(|_| "compositor health failed")?;
    let (width, height) = super::display::dimensions()?;
    let (backing_va, stride, byte_len) = super::backing::alloc(width, height)?;
    let ctx = Context::new(
        keyring_port,
        desktop_shell_port,
        compositor_port,
        width,
        height,
        stride,
        backing_va,
    );
    render::paint_locked(&ctx);
    let handle = match super::register::surface(width, height, stride, byte_len, backing_va) {
        Ok(handle) => handle,
        Err(e) => {
            super::cleanup_backing::cleanup_backing(backing_va, byte_len)?;
            return Err(e);
        }
    };
    if compositor::push_scene_submit(
        compositor_port,
        1,
        handle,
        0,
        0,
        width,
        height,
        super::constants::OVERLAY_Z,
    )
    .is_err()
    {
        super::cleanup_surface::cleanup_surface(handle)?;
        super::cleanup_backing::cleanup_backing(backing_va, byte_len)?;
        return Err("compositor scene submit failed");
    }
    Ok(ctx)
}
