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

use crate::app::App;
use crate::clients::{compositor, wm};
use crate::discover::Peers;

use super::boot::BootedApp;
use super::request_id::next;

const APP_LAYER_Z: u32 = 2;

pub(super) fn apply_move<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    to_x: u32,
    to_y: u32,
) {
    let (w, h) = (booted.binding.width, booted.binding.height);
    let Ok(di) = compositor::display_info(peers.compositor, next(request_id)) else { return };
    let nx = to_x.min(di.width.saturating_sub(w));
    let ny = to_y.min(di.height.saturating_sub(h));
    let (ox, oy) = (booted.binding.x, booted.binding.y);
    if nx == ox && ny == oy {
        return;
    }
    let submitted = compositor::scene_submit(
        peers.compositor,
        next(request_id),
        booted.binding.surface_handle,
        nx,
        ny,
        w,
        h,
        APP_LAYER_Z,
    );
    if submitted.is_err() {
        return;
    }
    let _ = compositor::damage_commit(peers.compositor, next(request_id), ox, oy, w, h);
    let _ = wm::window_move(peers.wm, next(request_id), booted.manifest.window_id, nx, ny);
    booted.binding.x = nx;
    booted.binding.y = ny;
}
