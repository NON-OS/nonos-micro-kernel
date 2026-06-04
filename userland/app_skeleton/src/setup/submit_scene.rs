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

use crate::clients::compositor;
use crate::clients::wm::WindowPlacement;
use crate::discover::Peers;
use nonos_libc::mk_yield;

use super::request_id::bump;

const APP_LAYER_Z: u32 = 2;
const SCENE_SUBMIT_ATTEMPTS: usize = 8;

pub(super) fn submit_scene(
    peers: &Peers,
    surface_handle: u64,
    request_id: &mut u32,
    placement: WindowPlacement,
) -> Result<(), &'static str> {
    let mut last = "compositor rejected scene_submit";
    for _ in 0..SCENE_SUBMIT_ATTEMPTS {
        let rid = bump(request_id);
        match compositor::scene_submit(
            peers.compositor,
            rid,
            surface_handle,
            placement.x,
            placement.y,
            placement.width,
            placement.height,
            APP_LAYER_Z,
        ) {
            Ok(()) => return Ok(()),
            Err(e) => last = e,
        }
        mk_yield();
    }
    Err(last)
}
