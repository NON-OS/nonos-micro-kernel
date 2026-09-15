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

use nonos_app_skeleton::PaintBuffer;

use super::chip::{chip, CHIP_H};
use super::info_geom::InfoGeom;
use super::info_sizes::SEC_PX;
use super::info_tag_add::{add_box, ADD_LABEL};
use super::info_tag_chip::tag_chip;
use super::info_tag_geom::tag_slots;
use super::theme::INK3;

const HEADING: &str = "Tags";

/// The tags on the cursor path as a wrapped chip band, each chip carrying the
/// box that removes it, and an add chip closing the band. Chips wrap when the
/// next measured pill would overrun `g.w`; `info_tag_geom` decides where each
/// one lands and `info_tag_hit` reads the same pass back.
pub fn tag_band(fb: &mut PaintBuffer, g: &InfoGeom, names: &[&str]) {
    if g.chips_y + CHIP_H > g.bottom {
        return;
    }
    let _ = fb.text_ttf(g.x as i32, g.tags_head as i32, HEADING, INK3, SEC_PX);
    let slots = tag_slots(names, g.x, g.chips_y, g.w);
    for slot in &slots {
        tag_chip(fb, slot);
    }
    let (ax, ay, _) = add_box(&slots, g.x, g.chips_y, g.w);
    let _ = chip(fb, ax, ay, ADD_LABEL, false);
}
