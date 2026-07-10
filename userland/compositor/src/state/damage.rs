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

// Damage is tracked as a small set of separate rectangles rather than one
// bounding box. With a window open, damage arrives from far-apart places (the
// cursor, an app window, the top bar); merging them into one box made the
// compositor recomposite the whole screen, including the empty space between,
// every frame. Keeping them separate means only the pixels that actually
// changed are recomposited. Overlapping or touching rects still merge so the
// same pixels are never composited twice, and the list is bounded.

const MAX_RECTS: usize = 8;

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct DamageAccumulator {
    rects: [Rect; MAX_RECTS],
    count: usize,
}

impl DamageAccumulator {
    pub const fn new() -> Self {
        const EMPTY: Rect = Rect { x: 0, y: 0, width: 0, height: 0 };
        Self { rects: [EMPTY; MAX_RECTS], count: 0 }
    }

    pub fn mark_full(&mut self, width: u32, height: u32) {
        self.rects[0] = Rect { x: 0, y: 0, width, height };
        self.count = 1;
    }

    pub fn accumulate(&mut self, r: Rect) {
        if r.width == 0 || r.height == 0 {
            return;
        }
        // Fold into any rect it overlaps or touches, to avoid double work.
        for i in 0..self.count {
            if touches(self.rects[i], r) {
                self.rects[i] = union(self.rects[i], r);
                return;
            }
        }
        if self.count < MAX_RECTS {
            self.rects[self.count] = r;
            self.count += 1;
        } else {
            // Full: fold into whichever rect grows the least. Still correct
            // (the region stays covered), just a little less precise.
            let j = self.smallest_growth(r);
            self.rects[j] = union(self.rects[j], r);
        }
    }

    /// One damaged rectangle per call; the compositor loop drains the rest on
    /// following iterations.
    pub fn drain(&mut self) -> Option<Rect> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        Some(self.rects[self.count])
    }

    fn smallest_growth(&self, r: Rect) -> usize {
        let mut best = 0;
        let mut best_area = u64::MAX;
        for i in 0..self.count {
            let u = union(self.rects[i], r);
            let area = u.width as u64 * u.height as u64;
            if area < best_area {
                best_area = area;
                best = i;
            }
        }
        best
    }
}

// Two rects overlap or share an edge, so merging them wastes no coverage.
fn touches(a: Rect, b: Rect) -> bool {
    let ax1 = a.x.saturating_add(a.width);
    let ay1 = a.y.saturating_add(a.height);
    let bx1 = b.x.saturating_add(b.width);
    let by1 = b.y.saturating_add(b.height);
    a.x <= bx1 && b.x <= ax1 && a.y <= by1 && b.y <= ay1
}

fn union(a: Rect, b: Rect) -> Rect {
    let x0 = a.x.min(b.x);
    let y0 = a.y.min(b.y);
    let x1 = a.x.saturating_add(a.width).max(b.x.saturating_add(b.width));
    let y1 = a.y.saturating_add(a.height).max(b.y.saturating_add(b.height));
    Rect { x: x0, y: y0, width: x1 - x0, height: y1 - y0 }
}
