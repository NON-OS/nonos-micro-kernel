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


//! Draining the client's requests and answering what is understood.

use crate::linux::guest::Guest;

use super::object::Object;
use super::ops::req;
use super::route::route;
use super::unserved::unserved;
use super::args::Args;
use super::wire::next;

pub fn serve(guest: &mut Guest) {
    while one(guest).is_some() {}
}

/// One message. It leaves the queue before it is served, so a handler
/// that refuses cannot leave it there to be served again forever.
fn one(guest: &mut Guest) -> Option<()> {
    let (object, opcode, id, body, size) = {
        let (msg, size) = next(&guest.display.to_server)?;
        (guest.objects.get(msg.object), msg.opcode, msg.object, msg.args.to_vec(), size)
    };
    guest.display.to_server.drain(..size);
    let mut args = Args::new(&body);
    if !route(guest, object, id, opcode, &mut args) {
        unserved(object, opcode);
    }
    Some(())
}

/// Requests that only remove something.
pub fn is_destructor(object: Option<Object>, opcode: u16) -> bool {
    matches!(
        (object, opcode),
        (Some(Object::Buffer), req::BUFFER_DESTROY)
            | (Some(Object::ShmPool), req::POOL_DESTROY)
            | (Some(Object::Surface), req::SURFACE_DESTROY)
    )
}
