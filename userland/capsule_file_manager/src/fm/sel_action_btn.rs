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
use super::header_slots::{GLYPH_GAP, GLYPH_S, ICON_PAD, TOOL_H, TOOL_PX};
use super::icon_draw::draw;
use super::paint_tool_pill::text_y;
use super::sel_model::SelAction;
use super::sel_slots::SelSlot;
use super::theme::{INK3, PILL_INK, RED, R_CARD};

/// One action in the band. A dimmed control is unwired -- the crate has no
/// handler behind it and says so when clicked rather than looking live. Delete
/// is the one destructive entry and the only one that spends the red hue.
pub fn action(fb: &mut PaintBuffer, slot: &SelSlot, y: u32, labelled: bool) {
    pill_r(fb, slot.x, y, slot.w, TOOL_H, R_CARD, PillState::Idle);
    let ink = if !slot.wired {
        INK3
    } else if slot.action == SelAction::Delete {
        RED
    } else {
        PILL_INK
    };
    let gy = y + TOOL_H.saturating_sub(GLYPH_S) / 2;
    if !labelled {
        let gx = slot.x + slot.w.saturating_sub(GLYPH_S) / 2;
        draw(fb, slot.icon, gx, gy, GLYPH_S, ink);
        return;
    }
    draw(fb, slot.icon, slot.x + ICON_PAD, gy, GLYPH_S, ink);
    let pen = slot.x + ICON_PAD + GLYPH_S + GLYPH_GAP;
    let _ = fb.text_ttf(pen as i32, text_y(y), slot.label, ink, TOOL_PX);
}
