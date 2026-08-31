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

use super::metrics::PAD;

// x, y, w, h in real pixels of the live surface. Every geom module speaks this
// and nothing else, so a painter and a hit test cannot disagree about a shape.
pub type Rect = (u32, u32, u32, u32);

pub fn hit(r: Rect, x: i32, y: i32) -> bool {
    x >= r.0 as i32 && x < (r.0 + r.2) as i32 && y >= r.1 as i32 && y < (r.1 + r.3) as i32
}

pub fn index_at<F: Fn(usize) -> Rect>(count: usize, x: i32, y: i32, rect: F) -> Option<usize> {
    (0..count).find(|i| hit(rect(*i), x, y))
}

// The whole drawable area, inset by the window pad. `app_skeleton` already hands
// `paint` a sub-buffer of the content rect and rebases event coords onto the same
// origin, so the titlebar is not ours to subtract. Every screen starts here, so
// growing the window moves all of them together.
pub fn content(w: u32, h: u32) -> Rect {
    (PAD, PAD, w.saturating_sub(PAD * 2), h.saturating_sub(PAD * 2))
}

pub fn centred(outer: Rect, w: u32, h: u32) -> Rect {
    let cw = w.min(outer.2);
    let ch = h.min(outer.3);
    (outer.0 + outer.2.saturating_sub(cw) / 2, outer.1 + outer.3.saturating_sub(ch) / 2, cw, ch)
}

pub fn inset(r: Rect, d: u32) -> Rect {
    (r.0 + d, r.1 + d, r.2.saturating_sub(d * 2), r.3.saturating_sub(d * 2))
}

pub fn row(r: Rect, index: usize, h: u32, gap: u32) -> Rect {
    (r.0, r.1 + index as u32 * (h + gap), r.2, h)
}

pub fn column(r: Rect, index: usize, count: usize, gap: u32) -> Rect {
    let n = count.max(1) as u32;
    let w = r.2.saturating_sub(gap * (n - 1)) / n;
    (r.0 + index as u32 * (w + gap), r.1, w, r.3)
}
