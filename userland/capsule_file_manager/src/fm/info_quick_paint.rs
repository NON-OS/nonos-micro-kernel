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

use super::chrome_pill::{pill_r, PillState};
use super::header_slots::GLYPH_S;
use super::icon_draw::draw;
use super::info_geom::InfoGeom;
use super::info_quick::{quick_slots, Quick};
use super::info_sizes::{QUICK_H, SEC_PX};
use super::theme::{INK3, PILL_INK, RED, R_CARD};

const HEADING: &str = "Quick Actions";

/// The tile row under the panel head, drawn only when the panel is tall enough
/// to hold it. The hit-test applies the same clip, so a tile that was skipped is
/// never clickable.
pub fn paint_quick(fb: &mut PaintBuffer, g: &InfoGeom) {
    if g.quick_y + QUICK_H > g.bottom {
        return;
    }
    let _ = fb.text_ttf(g.x as i32, g.quick_head as i32, HEADING, INK3, SEC_PX);
    let gy = g.quick_y + QUICK_H.saturating_sub(GLYPH_S) / 2;
    for slot in quick_slots(g.x, g.w) {
        pill_r(fb, slot.x, g.quick_y, slot.w, QUICK_H, R_CARD, PillState::Idle);
        let ink = if slot.action == Quick::Delete { RED } else { PILL_INK };
        let gx = slot.x + slot.w.saturating_sub(GLYPH_S) / 2;
        draw(fb, slot.icon, gx, gy, GLYPH_S, ink);
    }
}
