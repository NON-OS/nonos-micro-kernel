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

use super::layout::{bottom_dock_rect, spotlight_rect};
use super::paint_bottom_taskbar;
use crate::state::Context;

mod clear_overlay;
mod constants;
mod paint_rect;

const BOTTOM_DOCK_ARGB: u32 = 0xFF1B_2030;
const SPOTLIGHT_ARGB: u32 = 0xFF14_1B26;
const PANEL_BORDER_ARGB: u32 = 0xFF2A_3446;

pub fn paint_chrome(ctx: &Context) {
    let frame_start = crate::frametime::begin();
    clear_overlay::clear_overlay(ctx);
    super::topbar::paint(ctx);
    super::desktop_icons::paint_desktop_icons(ctx);
    if ctx.taskbar.visible {
        paint_rect::paint_rect(ctx, bottom_dock_rect(ctx.width, ctx.height), BOTTOM_DOCK_ARGB);
        paint_rect::paint_border(
            ctx,
            bottom_dock_rect(ctx.width, ctx.height),
            PANEL_BORDER_ARGB,
            1,
        );
        paint_bottom_taskbar(ctx);
    }
    if ctx.spotlight.visible {
        paint_rect::paint_rect(ctx, spotlight_rect(ctx.width, ctx.height), SPOTLIGHT_ARGB);
    }
    // The Launchpad, when open, covers the whole desktop and its dock.
    if ctx.launchpad {
        super::launchpad::paint_launchpad(ctx);
    }
    // The right-click menu floats above everything else, dock included.
    super::desktop_menu::paint(ctx);
    // The consent modal draws last so it sits above every other layer.
    super::consent::paint_consent(ctx);
    super::pkg_consent::paint_pkg_consent(ctx);
    crate::frametime::end(frame_start);
}
