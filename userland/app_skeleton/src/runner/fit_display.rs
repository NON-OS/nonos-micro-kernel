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

//! Fit a window's initial geometry to the real display. Manifests are written
//! against an assumed screen; on a smaller panel an oversized window would
//! cover everything and an off-screen origin would hide it. Clamp the size to
//! the work area (with a small margin so the desktop stays visible) and pull
//! the origin back so the whole window is on screen, centring when it had to
//! shrink. Applies to normal windows only; docks and overlays place themselves.

use crate::app::{AppManifest, WindowKind};
use crate::clients::compositor;
use crate::discover::Peers;

use super::request_id::next;

const MENUBAR_H: u32 = 28;
// Largest a window opens relative to the display, as a percent. Below 100 so
// windows cascade with reachable titlebars instead of covering the screen.
const WINDOW_FRACTION: u32 = 74;

pub(super) fn fit_to_display(
    mut manifest: AppManifest,
    peers: &Peers,
    request_id: &mut u32,
) -> AppManifest {
    if manifest.kind != WindowKind::Normal {
        return manifest;
    }
    let Ok(di) = compositor::display_info(peers.compositor, next(request_id)) else {
        return manifest;
    };
    if di.width == 0 || di.height == 0 {
        return manifest;
    }
    // Cap a window to a fraction of the work area, not the whole thing, so two
    // windows never fully occlude each other and the desktop plus dock stay
    // visible. The window manager cascades them from here, and a visible
    // titlebar is what makes click-to-raise a real way to switch windows.
    let work_h = di.height.saturating_sub(MENUBAR_H);
    let max_w = (di.width * WINDOW_FRACTION / 100).max(480);
    let max_h = (work_h * WINDOW_FRACTION / 100).max(360);

    let shrunk = manifest.width > max_w || manifest.height > max_h;
    manifest.width = manifest.width.min(max_w);
    manifest.height = manifest.height.min(max_h);

    if shrunk {
        // The manifest's origin was chosen for the larger size; centre instead.
        manifest.initial_x = (di.width - manifest.width) / 2;
        manifest.initial_y = MENUBAR_H + (di.height - MENUBAR_H - manifest.height) / 2;
    } else {
        // Keep the requested origin but pull the window fully on screen.
        manifest.initial_x = manifest.initial_x.min(di.width - manifest.width);
        manifest.initial_y = manifest.initial_y.max(MENUBAR_H).min(di.height - manifest.height);
    }
    manifest
}
