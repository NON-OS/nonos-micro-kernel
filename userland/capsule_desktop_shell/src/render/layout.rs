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

use crate::state::spotlight::{SPOTLIGHT_HEIGHT, SPOTLIGHT_WIDTH};
use crate::state::LAUNCHER_APPS;

/// Measured `ui_font::line_h(UI_PX)`. The facade reads the face at run time, so the
/// bar records the line box it was sized against.
pub const UI_LINE_H: u32 = 17;
/// One `UI_PX` line box plus symmetric padding, so the type clears the tile border.
pub const MENUBAR_TILE_H: u32 = UI_LINE_H + 6;
pub const MENUBAR_HEIGHT: u32 = MENUBAR_TILE_H + 6;
// One slot per desktop app, plus a trailing slot for the Launchpad button.
pub const BOTTOM_DOCK_WIDTH: u32 =
    (LAUNCHER_APPS.len() as u32 + 1) * (TASKBAR_ENTRY_W + 6) - 6 + 24;
pub const BOTTOM_DOCK_HEIGHT: u32 = 64;
pub const BOTTOM_DOCK_BOTTOM_INSET: u32 = 24;
pub const TASKBAR_ENTRY_W: u32 = 80;

/// Left edge of the Launchpad button: the slot just past the last app.
pub fn launchpad_slot_x(dock: Rect) -> u32 {
    dock.x + 12 + LAUNCHER_APPS.len() as u32 * (TASKBAR_ENTRY_W + 6)
}

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn menubar_rect(display_width: u32) -> Rect {
    Rect { x: 0, y: 0, width: display_width, height: MENUBAR_HEIGHT }
}

pub fn bottom_dock_rect(display_width: u32, display_height: u32) -> Rect {
    let w = core::cmp::min(BOTTOM_DOCK_WIDTH, display_width);
    let h = core::cmp::min(BOTTOM_DOCK_HEIGHT, display_height);
    let x = display_width.saturating_sub(w) / 2;
    let y = display_height.saturating_sub(h + BOTTOM_DOCK_BOTTOM_INSET);
    Rect { x, y, width: w, height: h }
}

pub fn spotlight_rect(display_width: u32, display_height: u32) -> Rect {
    let w = core::cmp::min(SPOTLIGHT_WIDTH, display_width);
    let h = core::cmp::min(SPOTLIGHT_HEIGHT, display_height);
    let x = display_width.saturating_sub(w) / 2;
    let y = (display_height / 3).saturating_sub(h / 2);
    Rect { x, y, width: w, height: h }
}
