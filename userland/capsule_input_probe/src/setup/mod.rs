mod discover;
mod fill;

use nonos_libc::{
    mk_mmap, mk_surface_register, mk_surface_release, mk_surface_share, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};

use crate::clients::compositor;
use crate::clients::display_info;
use crate::debug;
use crate::state::Context;

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;
const OVERLAY_Z: u32 = 1;
const FILL_ARGB: u32 = 0x0020_3040;

pub fn run() -> Result<Context, &'static str> {
    let compositor_port = discover::lookup_compositor_port()?;
    let router_port = discover::lookup_router_port()?;
    compositor::healthcheck(compositor_port, 1).map_err(|_| "compositor health failed")?;
    let di = display_info::query_display_info(compositor_port, 3)
        .map_err(|_| "display info query failed")?;
    let width = di.width;
    let height = di.height;
    let stride = di.stride;
    let byte_len = (stride as u64).checked_mul(height as u64).ok_or("surface size overflow")?;
    let base =
        mk_mmap(core::ptr::null_mut(), byte_len as usize, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if base.is_null() {
        return Err("backing mmap failed");
    }
    let backing_va = base as u64;
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len,
        base_va: backing_va,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        return Err("surface register rejected");
    }
    let handle = mk_surface_share(sid as u64);
    if handle <= 0 {
        return Err("surface share rejected");
    }
    fill::fill(backing_va, width, height, stride, FILL_ARGB);
    let submit =
        compositor::push_scene_submit(compositor_port, 1, handle as u64, 0, 0, width, height, OVERLAY_Z);
    if submit.is_err() {
        let _ = mk_surface_release(handle as u64);
        return Err("compositor scene submit failed");
    }
    if compositor::damage_commit(compositor_port, 2, width, height).is_err() {
        let _ = mk_surface_release(handle as u64);
        return Err("compositor damage commit failed");
    }
    debug::marker(b"setup ok");
    Ok(Context::new(backing_va, width, height, stride, compositor_port, router_port))
}
