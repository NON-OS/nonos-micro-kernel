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

use crate::app::{App, AppManifest};
use crate::clients::compositor;
use crate::discover::Peers;
use crate::setup::{open_window, WindowBinding};
use nonos_libc::mk_yield;

use super::paint_frame::paint;
use super::request_id::next;

const INITIAL_PAINT_ATTEMPTS: usize = 256;

pub(super) struct BootedApp<A: App> {
    pub app: A,
    pub manifest: AppManifest,
    pub binding: WindowBinding,
    pub primed: bool,
}

pub(super) fn boot<A: App>(
    mut app: A,
    peers: &Peers,
    request_id: &mut u32,
) -> Result<BootedApp<A>, &'static str> {
    let manifest = app.manifest();
    let binding = open_window(peers, &manifest, request_id)?;
    let primed = prime_frame(&mut app, &manifest, &binding, peers, request_id);
    Ok(BootedApp { app, manifest, binding, primed })
}

fn prime_frame<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &WindowBinding,
    peers: &Peers,
    request_id: &mut u32,
) -> bool {
    for _ in 0..INITIAL_PAINT_ATTEMPTS {
        if paint_once(app, manifest, binding, peers.toolkit, peers.compositor, request_id) {
            return true;
        }
        mk_yield();
    }
    false
}

pub(super) fn paint_once<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &WindowBinding,
    toolkit_port: u32,
    compositor_port: u32,
    request_id: &mut u32,
) -> bool {
    if paint(app, manifest, binding, toolkit_port, request_id).is_err() {
        return false;
    }
    let rid = next(request_id);
    compositor::damage_commit(compositor_port, rid, 0, 0, manifest.width, manifest.height).is_ok()
}
