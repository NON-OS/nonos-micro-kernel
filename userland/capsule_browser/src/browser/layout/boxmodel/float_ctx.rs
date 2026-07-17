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

//! The floats active in one block container. Left and right floats shorten the
//! band available to following in-flow content until that content passes their
//! bottom edge; `clear` drops a box below them.

use alloc::vec::Vec;

use crate::browser::css::Clear;

struct FloatRect {
    left: i32,
    right: i32,
    bottom: i32,
    is_left: bool,
}

pub(super) struct FloatCtx {
    rects: Vec<FloatRect>,
    content_left: i32,
    content_right: i32,
}

impl FloatCtx {
    pub(super) fn new(content_left: i32, content_w: i32) -> Self {
        Self { rects: Vec::new(), content_left, content_right: content_left + content_w }
    }

    // The left and right inner edges available at row `y`, after the floats that
    // span it.
    fn edges_at(&self, y: i32) -> (i32, i32) {
        let mut left = self.content_left;
        let mut right = self.content_right;
        for r in &self.rects {
            if y < r.bottom {
                if r.is_left {
                    left = left.max(r.right);
                } else {
                    right = right.min(r.left);
                }
            }
        }
        (left, right)
    }

    // The in-flow band at `y`: its left edge and width.
    pub(super) fn band(&self, y: i32) -> (i32, i32) {
        let (l, r) = self.edges_at(y);
        (l, (r - l).max(0))
    }

    // The lowest row at or below `y` where a `w`-wide float of the given side
    // fits between the existing floats.
    fn fit_row(&self, mut y: i32, w: i32) -> i32 {
        loop {
            let (l, r) = self.edges_at(y);
            if r - l >= w {
                return y;
            }
            // Drop to the nearest float bottom below y and try again.
            let next = self.rects.iter().filter(|f| f.bottom > y).map(|f| f.bottom).min();
            match next {
                Some(b) => y = b,
                None => return y,
            }
        }
    }

    // The top-left corner a `w`-wide float of the given side takes at or below
    // `y`, without recording it (its height is not known until it is laid out).
    pub(super) fn next_pos(&self, is_left: bool, w: i32, y: i32) -> (i32, i32) {
        let row = self.fit_row(y, w);
        let (l, r) = self.edges_at(row);
        let x = if is_left { l } else { (r - w).max(l) };
        (x, row)
    }

    // Record a laid-out float so later content flows around it.
    pub(super) fn record(&mut self, is_left: bool, x: i32, w: i32, bottom: i32) {
        self.rects.push(FloatRect { left: x, right: x + w, bottom, is_left });
    }

    // The row a box with `clear` must start at to sit below the floats it clears.
    pub(super) fn clear_row(&self, clear: Clear, y: i32) -> i32 {
        let mut row = y;
        for r in &self.rects {
            let clears = match clear {
                Clear::Both => true,
                Clear::Left => r.is_left,
                Clear::Right => !r.is_left,
                Clear::None => false,
            };
            if clears {
                row = row.max(r.bottom);
            }
        }
        row
    }

    // The lowest bottom of any float, so the container grows to contain them.
    pub(super) fn max_bottom(&self) -> i32 {
        self.rects.iter().map(|r| r.bottom).max().unwrap_or(i32::MIN)
    }
}
