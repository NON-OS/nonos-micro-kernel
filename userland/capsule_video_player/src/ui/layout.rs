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

pub const TOP: u32 = 28;
pub const EDGE: u32 = 10;
pub const BAR_H: u32 = 64;
pub const ROW_H: u32 = 44;

const BTN: u32 = 36;
const GAP: u32 = 12;
const PAD: u32 = 16;

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let (x, y) = (x as u32, y as u32);
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

pub struct Layout {
    pub video: Rect,
    pub bar: Rect,
    pub scrub: Rect,
    pub back: Rect,
    pub play: Rect,
    pub fwd: Rect,
    pub elapsed_x: u32,
    pub remain_x: u32,
}

pub fn layout(w: u32, h: u32) -> Layout {
    let usable_h = h.saturating_sub(TOP).saturating_sub(EDGE);
    let bar_h = BAR_H.min(usable_h);
    let bar_y = h.saturating_sub(EDGE).saturating_sub(bar_h).max(TOP);
    let inner_w = w.saturating_sub(EDGE).saturating_sub(PAD).saturating_sub(PAD).max(1);
    let cx = w / 2;
    Layout {
        video: Rect { x: 0, y: TOP, w, h: bar_y.saturating_sub(TOP) },
        bar: Rect { x: 0, y: bar_y, w: w.saturating_sub(EDGE), h: bar_h },
        scrub: Rect { x: PAD, y: bar_y + 8, w: inner_w, h: 6 },
        back: Rect { x: cx.saturating_sub(BTN + GAP + BTN / 2), y: bar_y + 22, w: BTN, h: BTN },
        play: Rect { x: cx.saturating_sub(BTN / 2), y: bar_y + 22, w: BTN, h: BTN },
        fwd: Rect { x: cx + BTN / 2 + GAP, y: bar_y + 22, w: BTN, h: BTN },
        elapsed_x: PAD,
        remain_x: w.saturating_sub(EDGE).saturating_sub(PAD),
    }
}
