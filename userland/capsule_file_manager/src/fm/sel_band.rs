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

//! The band that appears over the listing once entries are checked: what is
//! selected, how big it is, and what can be done to it. Every box it draws comes
//! from `sel_geom` and `sel_slots`, which `sel_hit` tests against.

use nonos_app_skeleton::PaintBuffer;

use super::chrome_card::Plate;
use super::header_slots::{GLYPH_GAP, GLYPH_S};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::measure_text::truncate_to_width;
use super::sel_action_btn::action;
use super::sel_geom::{action_y, band, summary_y, SelBand, PAD, SUM_PX};
use super::sel_slots::slots;
use super::sel_summary::summary;
use super::state::State;
use super::theme::{CY, HAIR_CY, INK, PANEL, R_CARD, TINT_BOT, TINT_TOP};

/// Draws nothing at all when the selection is empty, so the listing keeps its
/// whole height until there is something for the band to act on.
pub fn paint_band(state: &State, fb: &mut PaintBuffer) {
    if state.selected.is_empty() {
        return;
    }
    let b = band(fb.width, fb.height);
    Plate::new(PANEL)
        .radius(R_CARD)
        .line(HAIR_CY)
        .tint(TINT_TOP, TINT_BOT)
        .glow(6)
        .draw(fb, b.x, b.y, b.w, b.h);
    headline(state, fb, &b);
    let ay = action_y(&b);
    for slot in slots(&b) {
        action(fb, &slot, ay, b.labelled);
    }
}

fn headline(state: &State, fb: &mut PaintBuffer, b: &SelBand) {
    let sy = summary_y(b);
    draw(fb, Icon::Check, b.x + PAD, sy, GLYPH_S, CY);
    let x = b.x + PAD + GLYPH_S + GLYPH_GAP;
    let room = b.w.saturating_sub(PAD * 2 + GLYPH_S + GLYPH_GAP);
    let text = summary(state);
    let cut = truncate_to_width(fb, text.as_str(), SUM_PX, room);
    let _ = fb.text_ttf(x as i32, sy as i32, cut, INK, SUM_PX);
}
