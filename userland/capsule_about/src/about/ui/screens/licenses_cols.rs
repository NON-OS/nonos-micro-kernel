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

use crate::about::data::third_party::COMPONENTS;
use crate::about::theme::RULE_SOFT;

use super::super::kv::ROW_H;
use super::super::metrics::{BODY_PX, CARD_PAD, KV_GAP, NUM_PX};
use super::super::text;
use super::licenses_table::{HEAD_COMPONENT, HEAD_ROLE};

// Both column origins come from the widest cell the column actually holds, header
// included, measured in the face that cell is drawn with: the body face is
// proportional and the role and licence cells are mono, so a glyph count or a
// literal x would put the columns somewhere the text is not.
pub fn columns() -> (u32, u32) {
    let mut name_w = text::width_of(HEAD_COMPONENT, BODY_PX);
    let mut role_w = text::width_of(HEAD_ROLE, BODY_PX);
    for c in COMPONENTS.iter() {
        name_w = name_w.max(text::width_of(c.name, BODY_PX));
        role_w = role_w.max(text::width_of_mono(c.role, NUM_PX));
    }
    let role_x = CARD_PAD + name_w + KV_GAP;
    (role_x, role_x + role_w + KV_GAP)
}

// A zebra band is a rectangle and `blend_rect` takes unsigned coordinates, so a
// row that has scrolled past the pane top is drawn from the part of it that
// survives, or not at all. `fill_rect` would not blend and would punch the band
// straight through the card.
pub fn band(fb: &mut PaintBuffer, y: i32, w: u32) {
    let bottom = y + ROW_H as i32;
    if bottom <= 0 || y >= fb.height as i32 {
        return;
    }
    let top = y.max(0) as u32;
    fb.blend_rect(CARD_PAD, top, w, bottom as u32 - top, RULE_SOFT);
}
