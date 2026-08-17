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

use super::geom::sidebar_w;
use crate::ui::layout::{Rect, TOP};

pub const TOPBAR_H: u32 = 58;
pub const GUTTER: u32 = 20;

pub fn content(w: u32, h: u32) -> Rect {
    let x = sidebar_w(w);
    Rect { x, y: TOP, w: w.saturating_sub(x), h: h.saturating_sub(TOP) }
}

pub fn topbar(w: u32, h: u32) -> Rect {
    let c = content(w, h);
    Rect { x: c.x + GUTTER, y: c.y + 14, w: c.w.saturating_sub(GUTTER * 2), h: TOPBAR_H }
}

pub fn body(w: u32, h: u32) -> Rect {
    let c = content(w, h);
    let top = c.y + 14 + TOPBAR_H + 6;
    Rect {
        x: c.x + GUTTER,
        y: top,
        w: c.w.saturating_sub(GUTTER * 2),
        h: (c.y + c.h).saturating_sub(top).saturating_sub(GUTTER),
    }
}

pub fn search(w: u32, h: u32) -> Rect {
    let t = topbar(w, h);
    let width = t.w.saturating_sub(420).clamp(180, 460);
    Rect { x: t.x, y: t.y + 8, w: width, h: 38 }
}

pub fn tool(w: u32, h: u32, offset: u32, width: u32) -> Rect {
    let t = topbar(w, h);
    let right = t.x + t.w;
    Rect { x: right.saturating_sub(offset + width), y: t.y + 8, w: width, h: 38 }
}
