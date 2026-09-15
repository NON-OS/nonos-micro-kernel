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
use super::crumbs::{crumb_parts, crumb_slots};
use super::header_layout::tool_y;
use super::header_slots::{HeadHit, CRUMB_PAD, CRUMB_PX, CRUMB_SEP_W, HOME_W, TOOL_H};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::layout::{CONTENT_X, HEADER_H};
use super::paint_toolbar::paint_toolbar;
use super::paint_tool_pill::text_y;
use super::state::State;
use super::theme::{DEEP, HAIR, INK, INK2, INK3, LINE, PANEL, R_CARD, TINT_BOT, TINT_TOP};
use super::toolbar_buttons::glyph_y;

const TRAIL: Plate = Plate::new(PANEL).radius(R_CARD).line(HAIR).tint(TINT_TOP, TINT_BOT);

// The separator chevron is drawn a size down from a control glyph so it reads as
// punctuation between segments rather than as another button.
const SEP_S: u32 = 12;

pub fn paint_header(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width;
    let cw = w.saturating_sub(CONTENT_X);
    fb.fill_rect(CONTENT_X, 0, cw, HEADER_H, DEEP);
    fb.fill_rect(CONTENT_X, HEADER_H - 1, cw, 1, LINE);
    paint_crumbs(state, fb);
    paint_toolbar(state, fb);
}

// The capsule and its separators are derived from the very slots the segments
// were laid out into, so the measured layout stays the only source of geometry.
fn paint_crumbs(state: &State, fb: &mut PaintBuffer) {
    let slots = crumb_slots(state);
    let (Some(first), Some(last)) = (slots.first(), slots.last()) else { return };
    let parts = crumb_parts(state);
    let y = tool_y();
    let x = first.x.saturating_sub(CRUMB_PAD);
    TRAIL.draw(fb, x, y, (last.x + last.w + CRUMB_PAD).saturating_sub(x), TOOL_H);
    let tail = slots.len() - 1;
    for (n, slot) in slots.iter().enumerate() {
        let HeadHit::Crumb(i) = slot.hit else { continue };
        if n > 0 {
            let cx = slot.x.saturating_sub(CRUMB_SEP_W - (CRUMB_SEP_W - SEP_S) / 2);
            draw(fb, Icon::Chevron, cx, glyph_y(y, SEP_S), SEP_S, INK3);
        }
        if i == 0 {
            draw(fb, Icon::Home, slot.x, glyph_y(y, HOME_W), HOME_W, INK2);
            continue;
        }
        let Some(text) = parts.get(i) else { continue };
        let ink = if n == tail { INK } else { INK2 };
        let _ = fb.text_ttf(slot.x as i32, text_y(y), text, ink, CRUMB_PX);
    }
}
