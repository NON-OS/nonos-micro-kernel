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

use super::chrome_pill::{pill_ink, pill_r, PillState};
use super::header_slots::{
    Slot, GLYPH_GAP, GLYPH_S, ICON_PAD, NEW_LABEL, TOOL_H, TOOL_PX,
};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::paint_tool_pill::text_y;
use super::theme::R_CARD;

/// A pill that leads with a glyph and follows it with a label, laid out from the
/// same padding `icon_pill_w` measured the slot with.
pub fn icon_label(
    fb: &mut PaintBuffer,
    slot: &Slot,
    y: u32,
    icon: Icon,
    label: &str,
    state: PillState,
) {
    pill_r(fb, slot.x, y, slot.w, TOOL_H, R_CARD, state);
    let ink = pill_ink(state);
    let gy = y + TOOL_H.saturating_sub(GLYPH_S) / 2;
    draw(fb, icon, slot.x + ICON_PAD, gy, GLYPH_S, ink);
    let pen = slot.x + ICON_PAD + GLYPH_S + GLYPH_GAP;
    let _ = fb.text_ttf(pen as i32, text_y(y), label, ink, TOOL_PX);
}

/// The one accented control in the header: a cyan-hairlined pill that starts a
/// new file. It is drawn active at rest, which is what marks it as the row's
/// primary action rather than as a selected state.
pub fn new_btn(fb: &mut PaintBuffer, slot: &Slot, y: u32) {
    icon_label(fb, slot, y, Icon::Plus, NEW_LABEL, PillState::Active);
}
