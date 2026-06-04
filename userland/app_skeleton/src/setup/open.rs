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

use crate::app::AppManifest;
use crate::discover::Peers;
use nonos_libc::{mk_munmap, mk_surface_release};

use super::announce::announce;
use super::backing::alloc_backing;
use super::binding::WindowBinding;
use super::register::register_and_share;

pub fn open_window(
    peers: &Peers,
    manifest: &AppManifest,
    request_id: &mut u32,
) -> Result<WindowBinding, &'static str> {
    let (backing_va, stride, byte_len) = alloc_backing(manifest.width, manifest.height)?;
    let surface_handle =
        match register_and_share(backing_va, manifest.width, manifest.height, stride, byte_len) {
            Ok(handle) => handle,
            Err(e) => {
                if mk_munmap(backing_va as *mut u8, byte_len as usize) < 0 {
                    return Err("backing munmap failed");
                }
                return Err(e);
            }
        };
    let placement = match announce(peers, manifest, surface_handle, request_id) {
        Ok(p) => p,
        Err(e) => {
            if mk_surface_release(surface_handle) < 0 {
                return Err("surface release failed");
            }
            if mk_munmap(backing_va as *mut u8, byte_len as usize) < 0 {
                return Err("backing munmap failed");
            }
            return Err(e);
        }
    };
    Ok(WindowBinding {
        surface_handle,
        backing_va,
        x: placement.x,
        y: placement.y,
        width: placement.width,
        height: placement.height,
        stride_words: stride,
        byte_len,
    })
}
