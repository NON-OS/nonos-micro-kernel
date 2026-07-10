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
use crate::setup::reopen_surface;

use super::boot::BootedApp;
use super::repaint::repaint;
use super::request_id::next;

/// Reallocate the window's surface to `w`x`h` (keeping the origin), tell the
/// window manager, and repaint. Same surface-reopen path maximize uses, so the
/// app simply repaints at its new `fb.width`/`fb.height`.
pub(super) fn apply_resize<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    w: u32,
    h: u32,
) {
    // Bound the target by the display so a wild drag cannot allocate a surface
    // larger than the screen.
    let (mut w, mut h) = (w, h);
    if let Ok(di) = compositor::display_info(peers.compositor, next(request_id)) {
        if di.width > 0 && di.height > 0 {
            w = w.min(di.width);
            h = h.min(di.height);
        }
    }
    if w == booted.binding.width && h == booted.binding.height {
        return;
    }
    let (x, y) = (booted.binding.x, booted.binding.y);
    if let Ok(b) = reopen_surface(peers, &booted.binding, x, y, w, h, request_id) {
        booted.binding = b;
        let _ = wm::window_resize(peers.wm, next(request_id), booted.manifest.window_id, w, h);
        booted.maximized = false;
        repaint(booted, peers, request_id);
    }
}
