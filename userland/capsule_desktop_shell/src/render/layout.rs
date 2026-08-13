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
use super::ui_font;

const UI_LINE_H_LOGICAL: u32 = 17;
const TASKBAR_ENTRY_W_LOGICAL: u32 = 80;
const DOCK_GAP_LOGICAL: u32 = 6;
const DOCK_PAD_LOGICAL: u32 = 12;

/// Measured `ui_font::line_h(UI_PX)`. The facade reads the face at run time, so the
/// bar records the line box it was sized against.
pub fn ui_line_h() -> u32 {
    UI_LINE_H_LOGICAL * ui_font::scale()
}

/// One `UI_PX` line box plus symmetric padding, so the type clears the tile border.
pub fn menubar_tile_h() -> u32 {
    (UI_LINE_H_LOGICAL + 6) * ui_font::scale()
}

pub fn menubar_height() -> u32 {
    (UI_LINE_H_LOGICAL + 12) * ui_font::scale()
}

/// One slot per desktop app, plus a trailing slot for the Launchpad button.
pub fn bottom_dock_width() -> u32 {
    let slots = LAUNCHER_APPS.len() as u32 + 1;
    (slots * (TASKBAR_ENTRY_W_LOGICAL + DOCK_GAP_LOGICAL) - DOCK_GAP_LOGICAL + 24)
        * ui_font::scale()
}

pub fn bottom_dock_height() -> u32 {
    64 * ui_font::scale()
}

pub fn bottom_dock_bottom_inset() -> u32 {
    24 * ui_font::scale()
}

pub fn taskbar_entry_w() -> u32 {
    TASKBAR_ENTRY_W_LOGICAL * ui_font::scale()
}

/// Left edge of the Launchpad button: the slot just past the last app.
pub fn launchpad_slot_x(dock: Rect) -> u32 {
    let stride = (TASKBAR_ENTRY_W_LOGICAL + DOCK_GAP_LOGICAL) * ui_font::scale();
    dock.x + DOCK_PAD_LOGICAL * ui_font::scale() + LAUNCHER_APPS.len() as u32 * stride
}

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn menubar_rect(display_width: u32) -> Rect {
    Rect { x: 0, y: 0, width: display_width, height: menubar_height() }
}

pub fn bottom_dock_rect(display_width: u32, display_height: u32) -> Rect {
    let w = core::cmp::min(bottom_dock_width(), display_width);
    let h = core::cmp::min(bottom_dock_height(), display_height);
    let x = display_width.saturating_sub(w) / 2;
    let y = display_height.saturating_sub(h + bottom_dock_bottom_inset());
    Rect { x, y, width: w, height: h }
}

pub fn spotlight_rect(display_width: u32, display_height: u32) -> Rect {
    let w = core::cmp::min(SPOTLIGHT_WIDTH, display_width);
    let h = core::cmp::min(SPOTLIGHT_HEIGHT, display_height);
    let x = display_width.saturating_sub(w) / 2;
    let y = (display_height / 3).saturating_sub(h / 2);
    Rect { x, y, width: w, height: h }
}
