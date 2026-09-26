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

//! `wl_shm`: the pool a client draws into, and the buffers inside it.

use crate::linux::file::mapped_at;
use crate::linux::guest::Guest;

use super::args::Args;
use super::object::Object;
use super::ops::{ev, FORMAT_ARGB8888, FORMAT_XRGB8888};
use super::out::Event;
use super::state::Pool;

/// A client reads the format list before it asks for anything, and one it was
/// not offered is a protocol error on its side, so only the two that are
/// actually blitted are advertised.
pub fn formats(guest: &mut Guest, id: u32) {
    for format in [FORMAT_ARGB8888, FORMAT_XRGB8888] {
        Event::new(id, ev::SHM_FORMAT).u32(format).send(&mut guest.display.to_client);
    }
}

/// The descriptor was passed in the control data of the sendmsg that
/// carried this request, so it is taken from the queue in order.
pub fn create_pool(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(id), Some(_fd_slot), Some(size)) = (args.u32(), args.u32(), args.u32()) else {
        return;
    };
    let Some(fd) = take_fd(guest) else {
        return;
    };
    let Some((at, _)) = mapped_at(guest, fd as u64) else {
        /*
         * A pool over a descriptor the client never mapped has no pixels to
         * read, and reading zero would show a black window rather than say
         * why.
         */
        return;
    };
    /*
     * The declared size, bounded by what the client actually has at that
     * address.
     */
    let size = (size as u64).min(guest.mapped_from(at));
    if size == 0 {
        return;
    }
    guest.objects.put(id, Object::ShmPool);
    guest.scene.pools.push(Pool { id, at, size });
}

fn take_fd(guest: &mut Guest) -> Option<u32> {
    match guest.display.fds.is_empty() {
        true => None,
        false => Some(guest.display.fds.remove(0)),
    }
}
