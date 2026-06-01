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

use nonos_desktop::{scene_submit, subscribe, window_open, Peers};

const APP_LAYER_Z: u32 = 2;
const INPUT_MASK: u32 = 0xFF;

pub(super) fn announce(peers: &Peers, window_id: u32, handle: u64, width: u32, height: u32) -> bool {
    if window_open(peers.wm, 1, window_id, 0, 0, 0, width, height).is_err() {
        return false;
    }
    if scene_submit(peers.compositor, 2, handle, 0, 0, width, height, APP_LAYER_Z).is_err() {
        return false;
    }
    subscribe(peers.input_router, 3, INPUT_MASK).is_ok()
}
