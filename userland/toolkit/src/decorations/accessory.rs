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

use super::frame_rect::titlebar_rect;
use super::metrics::{ACCESSORY_INSET, ACCESSORY_PAD_Y};
use super::rect::Rect;

pub fn accessory_rect(w: u32, h: u32, maximized: bool, want_w: u32) -> Option<Rect> {
    if want_w == 0 {
        return None;
    }
    let t = titlebar_rect(w, h, maximized);
    let inner_h = t.h.saturating_sub(ACCESSORY_PAD_Y * 2);
    let avail_w = t.w.saturating_sub(ACCESSORY_INSET * 2);
    let aw = want_w.min(avail_w);
    if aw == 0 || inner_h == 0 {
        return None;
    }
    Some(Rect { x: t.x + t.w - ACCESSORY_INSET - aw, y: t.y + ACCESSORY_PAD_Y, w: aw, h: inner_h })
}
