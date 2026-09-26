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


//! Which handler a request belongs to. False means nothing served it.

use crate::linux::guest::Guest;

use super::object::Object;
use super::ops::req;
use super::serve::is_destructor;
use super::args::Args;
use super::{buffer, commit, handlers, seat, shm, surface, xdg};

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    opcode: u16,
    args: &mut Args<'_>,
) -> bool {
    match (object, opcode) {
        (Some(Object::Display), req::DISPLAY_SYNC) => handlers::sync(guest, args),
        (Some(Object::Display), req::DISPLAY_GET_REGISTRY) => handlers::registry(guest, args),
        (Some(Object::Registry), req::REGISTRY_BIND) => handlers::bind(guest, args),
        (Some(Object::Compositor), req::COMPOSITOR_CREATE_SURFACE) => surface::create(guest, args),
        (Some(Object::Shm), req::SHM_CREATE_POOL) => shm::create_pool(guest, args),
        (Some(Object::ShmPool), req::POOL_CREATE_BUFFER) => buffer::create_buffer(guest, id, args),
        (Some(Object::Surface), req::SURFACE_ATTACH) => surface::attach(guest, id, args),
        (Some(Object::Surface), req::SURFACE_FRAME) => surface::frame(guest, id, args),
        (Some(Object::Surface), req::SURFACE_COMMIT) => commit::commit(guest, id),
        (Some(Object::Surface), req::SURFACE_DAMAGE | req::SURFACE_DAMAGE_BUFFER) => {}
        (Some(Object::XdgBase), req::XDG_BASE_GET_SURFACE) => xdg::get_xdg_surface(guest, args),
        (Some(Object::XdgBase), req::XDG_BASE_PONG) => {}
        (Some(Object::XdgSurface), req::XDG_SURFACE_GET_TOPLEVEL) => {
            xdg::get_toplevel(guest, id, args)
        }
        (Some(Object::XdgSurface), req::XDG_SURFACE_ACK_CONFIGURE) => {
            xdg::ack_configure(guest, id, args)
        }
        (Some(Object::XdgSurface), req::XDG_SURFACE_SET_GEOMETRY) => {}
        (Some(Object::XdgToplevel), req::TOPLEVEL_SET_TITLE | req::TOPLEVEL_SET_APP_ID) => {}
        (Some(Object::Seat), req::SEAT_GET_POINTER) => seat::get_pointer(guest, args),
        (Some(Object::Seat), req::SEAT_GET_KEYBOARD) => seat::get_keyboard(guest, args),
        (o, c) if is_destructor(o, c) => guest.objects.drop_id(id),
        _ => return false,
    }
    true
}
