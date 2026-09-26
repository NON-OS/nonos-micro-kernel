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


//! Naming a request this shim does not serve yet.

use super::object::Object;

pub fn name(what: Object) -> &'static [u8] {
    match what {
        Object::Display => b"wl_display",
        Object::Registry => b"wl_registry",
        Object::Callback => b"wl_callback",
        Object::Compositor => b"wl_compositor",
        Object::Shm => b"wl_shm",
        Object::ShmPool => b"wl_shm_pool",
        Object::Buffer => b"wl_buffer",
        Object::Surface => b"wl_surface",
        Object::XdgBase => b"xdg_wm_base",
        Object::XdgSurface => b"xdg_surface",
        Object::XdgToplevel => b"xdg_toplevel",
        Object::Seat => b"wl_seat",
        Object::Pointer => b"wl_pointer",
        Object::Keyboard => b"wl_keyboard",
    }
}

/// A client that stalls should leave behind the interface and opcode it
/// was waiting on, because a protocol stall has no other symptom.
pub fn unserved(what: Option<Object>, opcode: u16) {
    let mut line = [0u8; 64];
    let head = b"[WAYLAND] unserved ";
    let tag = what.map(name).unwrap_or(b"unknown-object");
    let n = head.len();
    line[..n].copy_from_slice(head);
    let m = (n + tag.len()).min(line.len() - 4);
    line[n..m].copy_from_slice(&tag[..m - n]);
    line[m] = b'.';
    line[m + 1] = b'0' + (opcode / 10) as u8 % 10;
    line[m + 2] = b'0' + (opcode % 10) as u8;
    line[m + 3] = b'\n';
    let _ = nonos_libc::mk_debug(line.as_ptr(), m + 4);
}
