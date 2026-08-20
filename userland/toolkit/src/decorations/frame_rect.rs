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

use super::metrics::{
    BORDER_PX, FRAME_RADIUS, LIGHT_D, LIGHT_GAP, LIGHT_INSET, SHADOW_MARGIN, TITLEBAR_H,
};
use super::rect::Rect;

pub fn margin(maximized: bool) -> u32 {
    if maximized {
        0
    } else {
        SHADOW_MARGIN
    }
}

pub fn radius(maximized: bool) -> u32 {
    if maximized {
        0
    } else {
        FRAME_RADIUS
    }
}

pub fn frame_rect(w: u32, h: u32, maximized: bool) -> Rect {
    let m = margin(maximized);
    Rect { x: m, y: m, w: w.saturating_sub(m * 2), h: h.saturating_sub(m * 2) }
}

pub fn titlebar_rect(w: u32, h: u32, maximized: bool) -> Rect {
    let f = frame_rect(w, h, maximized);
    Rect { x: f.x, y: f.y, w: f.w, h: f.h.min(TITLEBAR_H) }
}

pub fn content_rect(w: u32, h: u32, maximized: bool) -> Rect {
    let f = frame_rect(w, h, maximized);
    Rect {
        x: f.x + BORDER_PX,
        y: f.y + TITLEBAR_H,
        w: f.w.saturating_sub(BORDER_PX * 2),
        h: f.h.saturating_sub(TITLEBAR_H + BORDER_PX),
    }
}

pub fn light_rect(i: u32, w: u32, h: u32, maximized: bool) -> Rect {
    let f = frame_rect(w, h, maximized);
    Rect {
        x: f.x + LIGHT_INSET + i * (LIGHT_D + LIGHT_GAP),
        y: f.y + (TITLEBAR_H - LIGHT_D) / 2,
        w: LIGHT_D,
        h: LIGHT_D,
    }
}
