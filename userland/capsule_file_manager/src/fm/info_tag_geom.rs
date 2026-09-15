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

extern crate alloc;

use alloc::vec::Vec;

use super::chip::{chip_w, CHIP_GAP, CHIP_H};

// The remove affordance's box and the gap holding it off the label. Both live
// inside the chip, so the pill the painter fills and the box the hit-test reads
// come out of the same width.
pub const X_BOX: u32 = 18;
pub const X_GAP: u32 = 4;

/// One laid-out tag chip: the pill, and the box inside it that removes the tag.
pub struct TagSlot<'a> {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub name: &'a str,
}

/// The pill's full width: the label measured off the free function, plus the
/// remove box it carries on its right.
pub fn tag_w(name: &str) -> u32 {
    chip_w(name) + X_GAP + X_BOX
}

/// Left edge of a chip's remove box.
pub fn remove_x(slot: &TagSlot) -> u32 {
    slot.x + slot.w.saturating_sub(X_BOX + X_GAP)
}

/// The chips wrapped into `w`, in one pass the painter draws from and the
/// hit-test resolves against.
pub fn tag_slots<'a>(names: &[&'a str], x: u32, top: u32, w: u32) -> Vec<TagSlot<'a>> {
    let mut out = Vec::new();
    let (mut cx, mut y) = (x, top);
    for name in names {
        let cw = tag_w(name);
        if cx > x && cx + cw > x + w {
            cx = x;
            y += CHIP_H + CHIP_GAP;
        }
        out.push(TagSlot { x: cx, y, w: cw, name });
        cx += cw + CHIP_GAP;
    }
    out
}
