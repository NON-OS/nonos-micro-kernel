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

use nonos_libc::{
    mk_mmap, mk_surface_attach, mk_yield, nonos_display_dimensions, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};

use super::discover;
use crate::gfx_client;
use crate::state::{
    AttachCache, Context, CursorTracker, DamageAccumulator, FocusTable, SceneTable,
};

const READY_ATTEMPTS: usize = 256;
const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub fn run() -> Result<Context, &'static str> {
    let mut last_err = "gfx primary unavailable";
    for _ in 0..READY_ATTEMPTS {
        match run_virtio_once() {
            Ok(ctx) => return Ok(ctx),
            Err(e) => {
                last_err = e;
                mk_yield();
            }
        }
    }
    if let Ok(ctx) = run_gop_once() {
        return Ok(ctx);
    }
    Err(last_err)
}

fn run_gop_once() -> Result<Context, &'static str> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let rc = nonos_display_dimensions(0, &mut width as *mut u32, &mut height as *mut u32);
    if rc != 0 || width == 0 || height == 0 {
        return Err("gop dimensions unavailable");
    }
    let stride = width.checked_mul(4).ok_or("gop stride overflow")?;
    let byte_len = (stride as u64).checked_mul(height as u64).ok_or("gop size overflow")?;
    let base =
        mk_mmap(core::ptr::null_mut(), byte_len as usize, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if base.is_null() {
        return Err("gop backing mmap failed");
    }
    let mut damage = DamageAccumulator::new();
    damage.mark_full(width, height);
    crate::debug::marker(b"[compositor] GOP-fb fallback mode");
    Ok(Context {
        gfx_port: 0,
        resource_id: 0,
        width,
        height,
        stride,
        backing_va: base as u64,
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
    let mut damage = DamageAccumulator::new();
    damage.mark_full(primary.width, primary.height);
    Ok(Context {
        gfx_port: gfx.port,
        resource_id: primary.resource_id,
        width: primary.width,
        height: primary.height,
        stride: primary.stride,
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
