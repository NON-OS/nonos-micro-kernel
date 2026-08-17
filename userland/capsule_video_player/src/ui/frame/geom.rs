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

use crate::ui::layout::{Rect, TOP};
use crate::ui::screen::NAV;

pub const SIDEBAR_W: u32 = 208;
pub const SIDEBAR_MIN: u32 = 62;
pub const COLLAPSE_AT: u32 = 880;
pub const NAV_H: u32 = 40;
pub const NAV_GAP: u32 = 4;
pub const NAV_TOP: u32 = 92;
pub const PAD: u32 = 12;
pub const STORAGE_H: u32 = 68;

pub fn collapsed(w: u32) -> bool {
    w < COLLAPSE_AT
}

pub fn sidebar_w(w: u32) -> u32 {
    if collapsed(w) {
        SIDEBAR_MIN
    } else {
        SIDEBAR_W
    }
}

pub fn sidebar(w: u32, h: u32) -> Rect {
    Rect { x: 0, y: TOP, w: sidebar_w(w), h: h.saturating_sub(TOP) }
}

pub fn brand(w: u32) -> Rect {
    Rect { x: PAD, y: TOP + 18, w: sidebar_w(w).saturating_sub(PAD * 2), h: 32 }
}

pub fn nav_item(w: u32, index: usize) -> Rect {
    let inner = sidebar_w(w).saturating_sub(PAD * 2);
    Rect {
        x: PAD,
        y: TOP + NAV_TOP + index as u32 * (NAV_H + NAV_GAP),
        w: inner,
        h: NAV_H,
    }
}

pub fn nav_hit(w: u32, x: i32, y: i32) -> Option<usize> {
    (0..NAV.len()).find(|i| nav_item(w, *i).contains(x, y))
}

pub fn storage(w: u32, h: u32) -> Rect {
    let sw = sidebar_w(w).saturating_sub(PAD * 2);
    let y = h.saturating_sub(PAD).saturating_sub(STORAGE_H).max(TOP);
    Rect { x: PAD, y, w: sw, h: STORAGE_H }
}
