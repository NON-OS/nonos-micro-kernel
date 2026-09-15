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

use super::chip::{chip_w, CHIP_GAP, CHIP_H};
use super::info_tag_geom::{tag_slots, TagSlot};

/// The add affordance rides in the tag band rather than in a row of its own:
/// the info strip is 268px wide and every fixed row it spends is a row the
/// permissions block loses on a 560px window.
pub const ADD_LABEL: &str = "+ Add tag";

/// The add chip's box, placed after the last tag and wrapped onto its own row
/// when it would overrun `w`. Painter and hit-test both call this.
pub fn add_box(slots: &[TagSlot], x: u32, top: u32, w: u32) -> (u32, u32, u32) {
    let aw = chip_w(ADD_LABEL);
    match slots.last() {
        Some(last) => {
            let cx = last.x + last.w + CHIP_GAP;
            if cx + aw > x + w {
                (x, last.y + CHIP_H + CHIP_GAP, aw)
            } else {
                (cx, last.y, aw)
            }
        }
        None => (x, top, aw),
    }
}

/// How tall the whole band runs, add chip included.
pub fn band_h(names: &[&str], w: u32) -> u32 {
    let slots = tag_slots(names, 0, 0, w);
    let (_, ay, _) = add_box(&slots, 0, 0, w);
    ay + CHIP_H
}
