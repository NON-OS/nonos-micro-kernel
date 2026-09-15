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

use super::metrics::S6;

pub const SIDEBAR_W: i32 = 236;
pub const RAIL_W: i32 = 300;
pub const TOPBAR_H: i32 = 58;
pub const TRANSPORT_H: i32 = 74;

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w: w.max(0), h: h.max(0) }
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }

    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    pub fn cx(&self) -> i32 {
        self.x + self.w / 2
    }

    pub fn cy(&self) -> i32 {
        self.y + self.h / 2
    }

    pub fn inset(&self, d: i32) -> Rect {
        Rect::new(self.x + d, self.y + d, self.w - d * 2, self.h - d * 2)
    }

    pub fn pad(&self, dx: i32, dy: i32) -> Rect {
        Rect::new(self.x + dx, self.y + dy, self.w - dx * 2, self.h - dy * 2)
    }

    pub fn row(&self, y: i32, h: i32) -> Rect {
        Rect::new(self.x, self.y + y, self.w, h)
    }

    pub fn cell(&self, i: i32, n: i32, gap: i32) -> Rect {
        let span = (self.w - gap * (n - 1)).max(0);
        let w = span / n.max(1);
        Rect::new(self.x + i * (w + gap), self.y, w, self.h)
    }

    pub fn centred(&self, w: i32, h: i32) -> Rect {
        Rect::new(self.x + (self.w - w) / 2, self.y + (self.h - h) / 2, w, h)
    }
}

pub struct Shell {
    pub frame: Rect,
    pub sidebar: Rect,
    pub topbar: Rect,
    pub content: Rect,
    pub rail: Rect,
    pub transport: Rect,
}

pub fn shell(width: u32, height: u32) -> Shell {
    let (w, h) = (width as i32, height as i32);
    let tr_y = h - TRANSPORT_H;
    let rail_w = if w >= 1080 { RAIL_W } else { 0 };
    let main_x = SIDEBAR_W;
    let main_w = w - SIDEBAR_W - rail_w;
    Shell {
        frame: Rect::new(0, 0, w, h),
        sidebar: Rect::new(0, 0, SIDEBAR_W, tr_y),
        topbar: Rect::new(main_x, 0, main_w, TOPBAR_H),
        content: Rect::new(main_x + S6, TOPBAR_H, main_w - S6 * 2, tr_y - TOPBAR_H),
        rail: Rect::new(main_x + main_w, 0, rail_w, tr_y),
        transport: Rect::new(0, tr_y, w, TRANSPORT_H),
    }
}

pub fn page(sh: &Shell) -> Rect {
    sh.content.pad(0, S6)
}
