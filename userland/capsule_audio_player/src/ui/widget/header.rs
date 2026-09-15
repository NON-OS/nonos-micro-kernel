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

//! Page and section headers, and the bare titled panel.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::metrics::{line_h, BODY, PAGE, R_CARD, S3, S4, SECTION};
use crate::ui::paint::{panel, text, text_right, width};
use crate::ui::theme::{EDGE, INK, MID, MUTE, RAISED};

pub fn page_header(fb: &mut PaintBuffer, r: Rect, title: &str, sub: &str, note: &str) -> i32 {
    text(fb, r.x, r.y, title, INK, PAGE);
    let mut y = r.y + line_h(PAGE);
    if !sub.is_empty() {
        text(fb, r.x, y, sub, MID, BODY);
        y += line_h(BODY);
    }
    if !note.is_empty() {
        let w = width(note, BODY);
        text(fb, r.right() - w, r.y + line_h(PAGE) - line_h(BODY), note, MUTE, BODY);
    }
    y - r.y + S3
}

pub fn section_header(fb: &mut PaintBuffer, r: Rect, title: &str, action: &str) {
    text(fb, r.x, r.y, title, INK, SECTION);
    if !action.is_empty() {
        text_right(fb, Rect::new(r.x, r.y, r.w, line_h(SECTION)), action, MUTE, BODY);
    }
}

pub fn card_panel(fb: &mut PaintBuffer, r: Rect, title: &str) -> Rect {
    panel(fb, r, R_CARD, RAISED, EDGE);
    let inner = r.pad(S4 + S3, S4);
    if title.is_empty() {
        return inner;
    }
    text(fb, inner.x, inner.y, title, INK, SECTION);
    Rect::new(inner.x, inner.y + line_h(SECTION) + S3, inner.w, inner.bottom() - inner.y - line_h(SECTION) - S3)
}
