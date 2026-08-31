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
use crate::clients::compositor;
use crate::discover::Peers;

use super::boot::BootedApp;
use super::paint_frame::paint;
use super::request_id::next;

pub(super) fn repaint<A: App>(booted: &mut BootedApp<A>, peers: &Peers, request_id: &mut u32) {
    let toolkit_rid = next(request_id);
    paint(
        &mut booted.app,
        &booted.manifest,
        &booted.binding,
        peers.toolkit,
        toolkit_rid,
        booted.drag.hover,
        booted.maximized,
    );
    let rid = next(request_id);
    let _ = compositor::damage_commit(
        peers.compositor,
        rid,
        booted.binding.x,
        booted.binding.y,
        booted.binding.width,
        booted.binding.height,
    );
}
