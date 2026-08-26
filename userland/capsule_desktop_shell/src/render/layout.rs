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

const TASKBAR_ENTRY_W_LOGICAL: u32 = 46;
const DOCK_GAP_LOGICAL: u32 = 7;
const DOCK_PAD_LOGICAL: u32 = 12;
const DOCK_BOX_INSET_LOGICAL: u32 = 9;
const DOCK_DIVIDER_LOGICAL: u32 = 11;
const MENUBAR_H_LOGICAL: u32 = 46;

pub fn menubar_height() -> u32 {
    MENUBAR_H_LOGICAL * ui_font::scale()
}

/// One slot per desktop app, plus a trailing slot for the Launchpad button.
pub fn bottom_dock_width() -> u32 {
    let slots = LAUNCHER_APPS.len() as u32 + 1;
    (slots * (TASKBAR_ENTRY_W_LOGICAL + DOCK_GAP_LOGICAL) - DOCK_GAP_LOGICAL
        + DOCK_DIVIDER_LOGICAL
        + 2 * DOCK_PAD_LOGICAL)
        * ui_font::scale()
}

pub fn bottom_dock_height() -> u32 {
    64 * ui_font::scale()
}

pub fn bottom_dock_bottom_inset() -> u32 {
    16 * ui_font::scale()
}

/// Width of the rule that separates the app run from the Launchpad slot,
/// margins included.
pub fn dock_divider_w() -> u32 {
    DOCK_DIVIDER_LOGICAL * ui_font::scale()
}

pub fn dock_gap() -> u32 {
    DOCK_GAP_LOGICAL * ui_font::scale()
}

pub fn dock_pad() -> u32 {
    DOCK_PAD_LOGICAL * ui_font::scale()
}

/// Vertical inset from the dock edge to the row of entry tiles.
pub fn dock_box_inset() -> u32 {
    DOCK_BOX_INSET_LOGICAL * ui_font::scale()
}

pub fn taskbar_entry_w() -> u32 {
    TASKBAR_ENTRY_W_LOGICAL * ui_font::scale()
}

/// Left edge of the Launchpad button: the slot just past the last app.
pub fn launchpad_slot_x(dock: Rect) -> u32 {
    let stride = (TASKBAR_ENTRY_W_LOGICAL + DOCK_GAP_LOGICAL) * ui_font::scale();
    dock.x
        + DOCK_PAD_LOGICAL * ui_font::scale()
        + LAUNCHER_APPS.len() as u32 * stride
        + dock_divider_w()
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
