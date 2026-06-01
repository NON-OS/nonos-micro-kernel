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

use nonos_libc::{mk_surface_attach, mk_yield, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};

use super::discover;
use crate::debug;
use crate::gfx_client;
use crate::state::{
    AttachCache, Context, CursorTracker, DamageAccumulator, FocusTable, SceneTable,
};

pub fn run() -> Result<Context, &'static str> {
    match run_virtio_once() {
        Ok(ctx) => Ok(ctx),
        Err(e) => {
            mk_yield();
            Err(e)
        }
    }
}

fn run_virtio_once() -> Result<Context, &'static str> {
    let gfx = discover::lookup_gfx_endpoint()?;
    let primary = gfx_client::get_primary_surface(gfx.port, 1)?;
    if primary.handle == 0 || primary.width == 0 || primary.height == 0 {
        return Err("gfx primary surface absent");
    }
    if primary.format != SURFACE_FORMAT_ARGB8888 {
        return Err("gfx primary surface format mismatch");
    }
    let mut desc = SurfaceDescriptor::default();
    let rc = mk_surface_attach(primary.handle, &mut desc);
    if rc <= 0 {
        return Err("surface attach rejected");
    }
    if desc.format != SURFACE_FORMAT_ARGB8888 {
        return Err("surface attach format mismatch");
    }
    if desc.width == 0 || desc.height == 0 {
        return Err("surface attach geometry missing");
    }
    let min_stride = desc.width.checked_mul(4).ok_or("surface attach stride overflow")?;
    if desc.stride < min_stride {
        return Err("surface attach stride too small");
    }
    let min_bytes = (desc.stride as u64)
        .checked_mul(desc.height as u64)
        .ok_or("surface attach byte_len overflow")?;
    if desc.byte_len < min_bytes {
        return Err("surface attach byte_len too small");
    }
    debug::marker_u64(b"primary w", desc.width as u64);
    debug::marker_u64(b"primary h", desc.height as u64);
    debug::marker_u64(b"primary stride", desc.stride as u64);
    debug::marker_u64(b"primary bytes", desc.byte_len);
    let mut damage = DamageAccumulator::new();
    damage.mark_full(desc.width, desc.height);
    Ok(Context {
        gfx_port: gfx.port,
        resource_id: primary.resource_id,
        width: desc.width,
        height: desc.height,
        stride: desc.stride,
        backing_len: desc.byte_len,
        backing_va: rc as u64,
        first_scanout_done: false,
        scanout_error_reported: false,
        next_request_id: 2,
        scene: SceneTable::new(),
        damage,
        focus: FocusTable::new(),
        cursor: CursorTracker::new(),
        attach: AttachCache::new(),
    })
}
