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
use crate::clients::wm;
use crate::clients::wm::WindowPlacement;
use crate::discover::Peers;

use super::input_mask::input_mask;
use super::request_id::bump;
use super::submit_scene::submit_scene;
use super::subscribe_input::subscribe_input;

pub(super) fn announce(
    peers: &Peers,
    manifest: &AppManifest,
    surface_handle: u64,
    request_id: &mut u32,
) -> Result<WindowPlacement, &'static str> {
    let rid = bump(request_id);
    let placement = wm::window_open(
        peers.wm,
        rid,
        manifest.window_id,
        manifest.kind as u32,
        manifest.initial_x,
        manifest.initial_y,
        manifest.width,
        manifest.height,
    )?;
    if let Err(e) = submit_scene(peers, surface_handle, request_id, placement) {
        let _ = wm::window_close(peers.wm, bump(request_id), manifest.window_id);
        return Err(e);
    }
    let _ = subscribe_input(peers.input_router, request_id, input_mask(manifest));
    Ok(placement)
}
