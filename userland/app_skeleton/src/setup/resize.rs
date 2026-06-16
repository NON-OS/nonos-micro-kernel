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

use nonos_libc::{mk_munmap, mk_surface_release};

use crate::clients::compositor;
use crate::clients::wm::WindowPlacement;
use crate::discover::Peers;

use super::backing::alloc_backing;
use super::binding::WindowBinding;
use super::register::register_and_share;
use super::request_id::bump;
use super::submit_scene::submit_scene;

pub fn reopen_surface(
    peers: &Peers,
    old: &WindowBinding,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    request_id: &mut u32,
) -> Result<WindowBinding, &'static str> {
    let rid = bump(request_id);
    let _ = compositor::scene_remove(peers.compositor, rid, 0);
    let (backing_va, stride, byte_len) = alloc_backing(w, h)?;
    let surface_handle = register_and_share(backing_va, w, h, stride, byte_len)?;
    submit_scene(peers, surface_handle, request_id, WindowPlacement { x, y, width: w, height: h })?;
    let _ = mk_surface_release(old.surface_handle);
    let _ = mk_munmap(old.backing_va as *mut u8, old.byte_len as usize);
    Ok(WindowBinding {
        surface_handle,
        backing_va,
        x,
        y,
        width: w,
        height: h,
        stride_words: w,
        byte_len,
    })
}
