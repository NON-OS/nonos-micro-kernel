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
use nonos_libc::mk_display_vsync_wait;

use super::boot::BootedApp;
use super::drain_ipc::drain;
use super::ensure_primed::ensure_primed;
use super::maximize;
use super::move_window::apply_move;
use super::refresh_input::refresh_input;
use super::repaint::repaint;
use super::request_id::next;
use super::teardown::close;

const APP_LAYER_Z: u32 = 2;

pub(super) fn service_frame<A: App>(
    booted: &mut BootedApp<A>,
    rx: &mut [u8],
    peers: &Peers,
    request_id: &mut u32,
) -> bool {
    refresh_input(booted, peers, request_id);
    if !ensure_primed(booted, peers, request_id) {
        let _ = mk_display_vsync_wait(0);
        return false;
    }
    let result = drain(
        &mut booted.app,
        &mut booted.drag,
        rx,
        booted.binding.width,
        booted.binding.x,
        booted.binding.y,
        peers.wm,
        booted.manifest.window_id,
        request_id,
    );
    if let Some((mx, my)) = result.move_to {
        apply_move(booted, peers, request_id, mx, my);
    }
    if result.close {
        return close(peers, booted.manifest.window_id, &booted.binding, request_id);
    }
    if result.minimize {
        let _ = wm::window_minimize(peers.wm, next(request_id), booted.manifest.window_id);
        let _ = compositor::scene_remove(peers.compositor, next(request_id), 0);
        booted.minimized = true;
        let _ = mk_display_vsync_wait(0);
        return false;
    }
    if result.restore && booted.minimized {
        let _ = compositor::scene_submit(
            peers.compositor,
            next(request_id),
            booted.binding.surface_handle,
            booted.binding.x,
            booted.binding.y,
            booted.binding.width,
            booted.binding.height,
            APP_LAYER_Z,
        );
        booted.minimized = false;
        repaint(booted, peers, request_id);
    }
    if result.maximize {
        maximize::toggle(booted, peers, request_id);
        let _ = mk_display_vsync_wait(0);
        return false;
    }
    if result.repaint {
        repaint(booted, peers, request_id);
    }
    let _ = mk_display_vsync_wait(0);
    false
}
