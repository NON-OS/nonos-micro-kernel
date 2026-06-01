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
use crate::clients::{compositor, input_router, wm};
use crate::discover::Peers;
use nonos_libc::mk_debug;

const APP_LAYER_Z: u32 = 2;

pub(super) fn announce(
    peers: &Peers,
    manifest: &AppManifest,
    surface_handle: u64,
    request_id: &mut u32,
) -> Result<(), &'static str> {
    let rid = bump(request_id);
    wm::window_open(
        peers.wm,
        rid,
        manifest.window_id,
        manifest.kind as u32,
        manifest.initial_x,
        manifest.initial_y,
        manifest.width,
        manifest.height,
    )?;
    let rid = bump(request_id);
    compositor::scene_submit(
        peers.compositor,
        rid,
        surface_handle,
        manifest.initial_x,
        manifest.initial_y,
        manifest.width,
        manifest.height,
        APP_LAYER_Z,
    )?;
    let rid = bump(request_id);
    if input_router::subscribe(peers.input_router, rid, manifest.input_kind_mask).is_err() {
        mk_debug(b"input subscribe deferred\n".as_ptr(), 24);
    }
    Ok(())
}

fn bump(slot: &mut u32) -> u32 {
    let id = *slot;
    *slot = slot.wrapping_add(1).max(1);
    id
}
