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

use nonos_app_skeleton::paint::PaintBuffer;

use super::button::Glyph;
use crate::ui::fit::{center_x, fit};
use crate::ui::layout::Rect;
use crate::ui::paint::rrect;
use crate::ui::text::{BODY_PX, TITLE_PX};
use crate::ui::theme;

const DISC: u32 = 76;
const ICON: u32 = 34;
const GAP: u32 = 20;

pub fn paint_empty(fb: &mut PaintBuffer, r: Rect, glyph: Glyph, head: &str, note: &str) {
    let block = DISC + GAP + 26 + 24;
    let top = r.y + r.h.saturating_sub(block) / 2;
    let cx = r.x + r.w / 2;
    rrect::fill_round(fb, cx.saturating_sub(DISC / 2), top, DISC, DISC, DISC / 2, theme::PANEL);
    rrect::stroke_round(
        fb,
        cx.saturating_sub(DISC / 2),
        top,
        DISC,
        DISC,
        DISC / 2,
        1,
        theme::BORDER,
    );
    glyph(
        fb,
        cx.saturating_sub(ICON / 2),
        top + DISC.saturating_sub(ICON) / 2,
        ICON,
        theme::TEXT_MUTED,
    );
    let room = r.w.saturating_sub(48).min(420);
    let head = fit(head, room, TITLE_PX);
    let y = top + DISC + GAP;
    fb.text_ttf(center_x(head, r.x, r.w, TITLE_PX), y as i32, head, theme::TEXT, TITLE_PX);
    let note = fit(note, room, BODY_PX);
    let ny = (y + 30) as i32;
    fb.text_ttf(center_x(note, r.x, r.w, BODY_PX), ny, note, theme::TEXT_MUTED, BODY_PX);
}

pub fn paint_inline(fb: &mut PaintBuffer, r: Rect, note: &str) {
    rrect::panel(fb, r.x, r.y, r.w, r.h, 10, theme::PANEL, theme::BORDER_SOFT);
    let note = fit(note, r.w.saturating_sub(28), BODY_PX);
    let y = crate::ui::text::center_y(r.y, r.h);
    fb.text_ttf(center_x(note, r.x, r.w, BODY_PX), y, note, theme::TEXT_MUTED, BODY_PX);
}
