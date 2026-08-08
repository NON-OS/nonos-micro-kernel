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

use super::display_list::DisplayList;

// Drop a laid-out item's fragments to their final row or line. Grid and flex
// lay every item at the container top and move it afterwards, once the row
// heights are known, so whatever was resolved against the old position has to
// travel with it. A clip the item established inside itself is in those old
// coordinates; a clip inherited from an ancestor is already in page
// coordinates and must stay where it is. `base` is the clip in force when the
// item was laid out, which is what tells the two apart.
//
// Where an ancestor clip is the tighter of the two on an edge, the item's own
// bound did not survive the intersection and cannot be recovered, so that
// edge stays put. That errs toward showing a little of what the ancestor was
// already clipping, never toward an item hiding its own content, which is the
// failure this exists to prevent: a card in the second row clipped against
// the first paints its background and nothing else.
pub(super) fn shift_down(
    frags: &mut DisplayList,
    a: usize,
    b: usize,
    dy: i32,
    base: Option<[i32; 4]>,
) {
    if dy == 0 {
        return;
    }
    let (top, bottom) = match base {
        Some(c) => (c[1], c[3]),
        None => (i32::MIN, i32::MAX),
    };
    for f in frags.iter_mut().take(b).skip(a) {
        f.y += dy;
        if let Some(c) = f.clip.as_mut() {
            if c[1] > top {
                c[1] += dy;
            }
            if c[3] < bottom {
                c[3] += dy;
            }
        }
    }
}
