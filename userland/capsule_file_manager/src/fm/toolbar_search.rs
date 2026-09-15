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

use super::chrome_card::Plate;
use super::header_layout::search_is_field;
use super::header_slots::{
    Slot, GLYPH_GAP, GLYPH_S, ICON_PAD, SEARCH_HINT, TOOL_H, TOOL_PX,
};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::measure_text::truncate_to_width;
use super::paint_tool_pill::text_y;
use super::state::State;
use super::theme::{HAIR, HAIR_CY, INK, INK3, PANEL, R_CARD};
use super::toolbar_buttons::{glyph_y, icon_btn};

const FIELD: Plate = Plate::new(PANEL).radius(R_CARD).line(HAIR);
const LIVE: Plate = Plate::new(PANEL).radius(R_CARD).line(HAIR_CY);

/// The query control. Where the header is too narrow for a field it collapses to
/// the bare glyph, which opens the same Search surface; where it is a field, the
/// live query is drawn in it and a clear mark is offered at its right.
pub fn search(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32, clear: Option<&Slot>) {
    if !search_is_field(state) {
        icon_btn(fb, slot, y, Icon::Magnifier, false, false);
        return;
    }
    let empty = state.query.is_empty();
    let plate = if empty { FIELD } else { LIVE };
    plate.draw(fb, slot.x, y, slot.w, TOOL_H);
    draw(fb, Icon::Magnifier, slot.x + ICON_PAD, glyph_y(y, GLYPH_S), GLYPH_S, INK3);
    let pen = slot.x + ICON_PAD + GLYPH_S + GLYPH_GAP;
    let tail = clear.map(|c| c.w).unwrap_or(ICON_PAD);
    let room = (slot.x + slot.w).saturating_sub(pen + tail);
    let text = if empty { SEARCH_HINT } else { state.query.as_str() };
    let ink = if empty { INK3 } else { INK };
    let cut = truncate_to_width(fb, text, TOOL_PX, room);
    let _ = fb.text_ttf(pen as i32, text_y(y), cut, ink, TOOL_PX);
    if let Some(slot) = clear {
        clear_mark(fb, slot, y);
    }
}

// The icon set carries no close glyph, so the clear affordance is stroked from
// the same anti-aliased primitive every icon is built out of.
fn clear_mark(fb: &mut PaintBuffer, slot: &Slot, y: u32) {
    let s = GLYPH_S / 2;
    let x = (slot.x + slot.w.saturating_sub(s) / 2) as i32;
    let top = glyph_y(y, s) as i32;
    fb.line_aa(x, top, x + s as i32, top + s as i32, INK3);
    fb.line_aa(x + s as i32, top, x, top + s as i32, INK3);
}
