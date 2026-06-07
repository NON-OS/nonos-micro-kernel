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
use crate::discover::Peers;
use nonos_libc::mk_display_vsync_wait;

use super::boot::BootedApp;
use super::drain_ipc::drain;
use super::ensure_primed::ensure_primed;
use super::refresh_input::refresh_input;
use super::repaint::repaint;
use super::teardown::close;

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
        rx,
        booted.manifest.width,
        peers.wm,
        booted.manifest.window_id,
        request_id,
    );
    if result.close {
        return close(peers, booted.manifest.window_id, &booted.binding, request_id);
    }
    if result.repaint {
        repaint(booted, peers, request_id);
    }
    let _ = mk_display_vsync_wait(0);
    false
}
