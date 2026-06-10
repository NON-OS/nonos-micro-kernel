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

const MENUBAR_H: u32 = 28;

pub(super) fn toggle<A: App>(booted: &mut BootedApp<A>, peers: &Peers, request_id: &mut u32) {
    if booted.maximized {
        let (x, y, w, h) = booted.saved;
        if let Ok(b) = reopen_surface(peers, &booted.binding, x, y, w, h, request_id) {
            booted.binding = b;
            let rid = next(request_id);
            let _ = wm::window_maximize(peers.wm, rid, booted.manifest.window_id, x, y, w, h);
            booted.maximized = false;
        }
    } else if let Ok(di) = compositor::display_info(peers.compositor, next(request_id)) {
        booted.saved =
            (booted.binding.x, booted.binding.y, booted.binding.width, booted.binding.height);
        let (x, y, w, h) = (0, MENUBAR_H, di.width, di.height.saturating_sub(MENUBAR_H));
        if let Ok(b) = reopen_surface(peers, &booted.binding, x, y, w, h, request_id) {
            booted.binding = b;
            let rid = next(request_id);
            let _ = wm::window_maximize(peers.wm, rid, booted.manifest.window_id, x, y, w, h);
            booted.maximized = true;
        }
    }
    repaint(booted, peers, request_id);
}
