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

use nonos_libc::{mk_exit, mk_munmap, mk_surface_release};

use crate::clients::{compositor, wm};
use crate::discover::Peers;
use crate::setup::WindowBinding;

use super::request_id::next;

pub(super) fn close(
    peers: &Peers,
    window_id: u32,
    binding: &WindowBinding,
    request_id: &mut u32,
) -> ! {
    let rid = next(request_id);
    if compositor::scene_remove(peers.compositor, rid, 0).is_err() {
        mk_exit(12);
    }
    if mk_surface_release(binding.surface_handle) < 0 {
        mk_exit(13);
    }
    if mk_surface_release(binding.surface_handle) < 0 {
        mk_exit(14);
    }
    if mk_munmap(binding.backing_va as *mut u8, binding.byte_len as usize) < 0 {
        mk_exit(15);
    }
    let rid = next(request_id);
    if wm::window_close(peers.wm, rid, window_id).is_err() {
        mk_exit(11);
    }
    mk_exit(0)
}
