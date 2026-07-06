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

use super::containing::Containing;

// Ambient layout state a box hands to its children: the containing block
// for absolute descendants, the active clip, and the stacking z.
#[derive(Clone, Copy)]
pub(super) struct Ctx {
    pub cb: Containing,
    pub clip: Option<[i32; 4]>,
    pub z: i32,
    // True inside a position:fixed subtree; its fragments pin on scroll.
    pub fixed: bool,
    // Inside a sticky subtree: the sticky box's flow y and its top offset,
    // so paint can clamp the whole subtree with one shift.
    pub sticky: Option<(i32, i32)>,
}

impl Ctx {
    // Intersect the active clip with `rect` ([x0, y0, x1, y1]).
    pub(super) fn clipped(mut self, rect: [i32; 4]) -> Self {
        self.clip = Some(match self.clip {
            Some(c) => [c[0].max(rect[0]), c[1].max(rect[1]), c[2].min(rect[2]), c[3].min(rect[3])],
            None => rect,
        });
        self
    }
}
