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

use super::rail_text::{clipped, left, lh, right, RAIL_GAP, RAIL_PAD};
use super::tokens::TAB_ACTIVE;
use crate::layout::Rect;
use crate::palette::Entry;
use crate::term::theme::types::Theme;

const RADIUS: u32 = 6;
const TAG_W: u32 = 34;

/// One result row: source tag, label, hint. The selected row is a blend over
/// the panel, never a raw fill, so the panel keeps showing through it.
pub fn draw_row(fb: &mut PaintBuffer, r: Rect, e: &Entry, sel: bool, t: &Theme) {
    if sel {
        fb.fill_round(r.x, r.y, r.w, r.h, RADIUS, TAB_ACTIVE);
    }
    let y = (r.y + r.h.saturating_sub(lh()) / 2) as i32;
    left(fb, r.x + RAIL_PAD, y, e.kind.tag(), t.dim);
    let hint_w = super::fit_text::width_of(fb, e.hint, super::rail_text::RAIL_PX);
    let x = r.x + RAIL_PAD + TAG_W;
    let room = (r.x + r.w).saturating_sub(x + hint_w + RAIL_GAP * 2 + RAIL_PAD);
    clipped(fb, x, y, room, e.label, if sel { t.accent } else { t.fg });
    right(fb, r.x + r.w.saturating_sub(RAIL_PAD), y, e.hint, t.dim);
}
