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
use super::chrome_pill::{pill_r, pill_ink, PillState};
use super::header_slots::{Slot, GLYPH_S, TOOL_H};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::theme::{HAIR, INK3, PANEL, R_CARD, TINT_BOT, TINT_TOP};

/// The ground a group of adjacent controls shares, so the group reads as one
/// track rather than as neighbouring buttons.
const GROUND: Plate =
    Plate::new(PANEL).radius(R_CARD).line(HAIR).tint(TINT_TOP, TINT_BOT);

/// Vertical origin that centres a `size`-square glyph in the control band.
pub fn glyph_y(y: u32, size: u32) -> u32 {
    y + TOOL_H.saturating_sub(size) / 2
}

/// The shared plate under the slots from `first` to `last` inclusive.
pub fn group_plate(fb: &mut PaintBuffer, first: &Slot, last: &Slot, y: u32) {
    GROUND.draw(fb, first.x, y, (last.x + last.w).saturating_sub(first.x), TOOL_H);
}

/// A bare icon control drawn onto a ground someone else laid down. The active
/// state raises its own inset plate, which is what makes a segmented toggle read
/// as one track with a lit segment. A dimmed button is unwired and inert.
pub fn icon_btn(fb: &mut PaintBuffer, slot: &Slot, y: u32, icon: Icon, on: bool, dim: bool) {
    let state = if on { PillState::Active } else { PillState::Idle };
    if on {
        pill_r(fb, slot.x + 2, y + 2, slot.w.saturating_sub(4), TOOL_H - 4, R_CARD - 4, state);
    }
    let ink = if dim { INK3 } else { pill_ink(state) };
    let gx = slot.x + slot.w.saturating_sub(GLYPH_S) / 2;
    draw(fb, icon, gx, glyph_y(y, GLYPH_S), GLYPH_S, ink);
}
