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

//! Getting the client's pixels onto a NONOS surface.

use crate::linux::guest::Guest;

use super::present_surface::surface;
use super::scene::Scene;

pub fn present(guest: &mut Guest, buffer: u32) {
    let Some((at, width, height, stride, bytes)) = source(&guest.scene, buffer) else {
        return;
    };
    /*
     * The descriptor handed to the kernel holds this buffer's address, so it
     * is allocated once and written in place afterwards.
     */
    if guest.scene.pixels.len() < bytes {
        if guest.scene.out.is_some() {
            return;
        }
        guest.scene.pixels.resize(bytes, 0);
    }
    let Some(src) = guest.read(at, bytes) else {
        return;
    };
    guest.scene.pixels[..bytes].copy_from_slice(&src);
    if let Some(handle) = surface(&mut guest.scene, width, height, stride) {
        let _ = nonos_libc::mk_surface_present_rect(handle, 0, 0, width, height);
    }
}

/// Where the pixels are, how they are shaped, and how many bytes that is.
fn source(scene: &Scene, buffer: u32) -> Option<(u64, u32, u32, u32, usize)> {
    let b = scene.buffers.iter().find(|b| b.id == buffer)?;
    let pool = scene.pools.iter().find(|p| p.id == b.pool)?;
    let at = pool.at.checked_add(b.offset)?;
    let bytes = (b.stride as u64).checked_mul(b.height as u64)?;
    if bytes == 0 || b.offset.checked_add(bytes)? > pool.size {
        return None;
    }
    Some((at, b.width, b.height, b.stride, bytes as usize))
}
