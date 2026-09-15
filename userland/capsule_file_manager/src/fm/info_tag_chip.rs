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
use super::info_tag_geom::{remove_x, TagSlot, X_BOX};
use super::theme::{INK3, RAISE};

// The icon set has no close mark, so the cross is stroked here from the same
// anti-aliased line primitive `icon_draw` uses, doubled on one axis for the
// weight every other glyph in the chrome carries.
const ARM: i32 = 5;

/// One tag as a pill that carries its own remove box. The pill is filled to the
/// full slot width first so the box sits inside the chip rather than beside it,
/// then `chip` lays the label down over its own share of that ground.
pub fn tag_chip(fb: &mut PaintBuffer, slot: &TagSlot) {
    fb.fill_round(slot.x, slot.y, slot.w, CHIP_H, CHIP_H / 2, RAISE);
    let _ = chip(fb, slot.x, slot.y, slot.name, false);
    cross(fb, remove_x(slot), slot.y);
}

fn cross(fb: &mut PaintBuffer, x: u32, y: u32) {
    let px = x as i32;
    let py = (y + CHIP_H.saturating_sub(X_BOX) / 2) as i32;
    let far = X_BOX as i32 - ARM;
    fb.line_aa(px + ARM, py + ARM, px + far, py + far, INK3);
    fb.line_aa(px + ARM + 1, py + ARM, px + far + 1, py + far, INK3);
    fb.line_aa(px + far, py + ARM, px + ARM, py + far, INK3);
    fb.line_aa(px + far - 1, py + ARM, px + ARM - 1, py + far, INK3);
}
