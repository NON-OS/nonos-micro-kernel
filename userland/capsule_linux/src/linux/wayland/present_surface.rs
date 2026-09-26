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


//! Registering the NONOS surface the pixels land on.

use nonos_app_skeleton::clients::compositor::scene_submit;
use nonos_app_skeleton::discover::lookup_port;
use nonos_libc::{mk_surface_register, SurfaceDescriptor};

use super::scene::Scene;

/// SURFACE_FORMAT_ARGB8888 in the surface registry, which is one there
/// and zero in wl_shm. The two numbers are unrelated and both are right.
const FORMAT_ARGB8888: u32 = 1;

/// Registered once, on the first commit: its size is the first buffer's,
/// and a client does not say before then.
pub fn surface(scene: &mut Scene, width: u32, height: u32, stride: u32) -> Option<u64> {
    if let Some(handle) = scene.out {
        return Some(handle);
    }
    let desc = SurfaceDescriptor {
        width,
        height,
        stride,
        format: FORMAT_ARGB8888,
        byte_len: scene.pixels.len() as u64,
        base_va: scene.pixels.as_ptr() as u64,
        flags: 0,
    };
    let handle = mk_surface_register(&desc);
    if handle < 0 {
        return None;
    }
    scene.out = Some(handle as u64);
    place(handle as u64, width, height);
    Some(handle as u64)
}

/// Registering a surface makes it exist; the compositor still has to be
/// told where it goes, or it is never drawn.
fn place(handle: u64, width: u32, height: u32) {
    let Some(port) = lookup_port(b"compositor") else {
        return;
    };
    let _ = scene_submit(port, 1, handle, 0, 0, width, height, 0);
}
