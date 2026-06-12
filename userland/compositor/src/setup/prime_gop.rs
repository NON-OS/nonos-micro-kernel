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

//! GOP-framebuffer display path. When no virtio-gpu driver announces a
//! gfx service, the compositor backs its primary surface with a
//! page-aligned buffer it owns, registers it, and presents through
//! MkSurfacePresent. The kernel blits that surface to the UEFI linear
//! framebuffer the bootloader handed off, so the desktop renders on any
//! UEFI machine: real hardware, VirtualBox, VMware, with no virtio-gpu.

use nonos_libc::{
    mk_mmap, mk_surface_attach, mk_surface_register, nonos_display_dimensions, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};

use crate::state::{
    AttachCache, Context, CursorTracker, DamageAccumulator, FocusTable, SceneTable,
};

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

pub fn run_gop_once() -> Result<Context, &'static str> {
    let (width, height) = display_dimensions()?;
    let stride = width.checked_mul(4).ok_or("gop: stride overflow")?;
    let byte_len = (stride as u64).checked_mul(height as u64).ok_or("gop: byte_len overflow")?;

    let base_va = alloc_backing(byte_len)?;
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len,
        base_va,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid <= 0 {
        return Err("gop: surface register rejected");
    }
    let handle = sid as u64;

    // Self-attach: the kernel returns this same mmap VA, which has a real
    // VMA the present path resolves against. We composite into it directly.
    let mut attached = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut attached);
    if va <= 0 || attached.format != SURFACE_FORMAT_ARGB8888 {
        return Err("gop: surface attach rejected");
    }

    let mut damage = DamageAccumulator::new();
    damage.mark_full(width, height);
    Ok(Context {
        gfx_port: 0,
        resource_id: 0,
        width,
        height,
        stride,
        backing_len: byte_len,
        backing_va: va as u64,
        gop_mode: true,
        surface_handle: handle,
        first_scanout_done: true,
        scanout_error_reported: false,
        next_request_id: 2,
        scene: SceneTable::new(),
        damage,
        focus: FocusTable::new(),
        cursor: CursorTracker::at(width / 2, height / 2),
        attach: AttachCache::new(),
    })
}

fn display_dimensions() -> Result<(u32, u32), &'static str> {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let rc = nonos_display_dimensions(0, &mut width as *mut u32, &mut height as *mut u32);
    if rc < 0 || width == 0 || height == 0 {
        return Err("gop: no display dimensions");
    }
    Ok((width, height))
}

// Back the surface with its own anonymous mapping, not the heap. mmap
// gives a dedicated, page-aligned VMA whose start is exactly base_va, so
// the kernel's present path resolves the surface to a real region. A
// heap allocation would sit inside the heap VMA and never match.
fn alloc_backing(byte_len: u64) -> Result<u64, &'static str> {
    let size = usize::try_from(byte_len).map_err(|_| "gop: backing too large")?;
    let ptr = mk_mmap(core::ptr::null_mut(), size, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    if ptr.is_null() {
        return Err("gop: backing mmap failed");
    }
    Ok(ptr as u64)
}
