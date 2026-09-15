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

//! Pill chips for filters, genres and metadata stamps.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::metrics::{pill, LABEL, S4};
use crate::ui::paint::{panel, text_centre, width};
use crate::ui::theme::{CYAN, CYAN_WASH, EDGE, MID, RAISED, VOID};

pub fn measure(label: &str) -> i32 {
    width(label, LABEL) + S4 * 2
}

pub fn chip(fb: &mut PaintBuffer, r: Rect, label: &str, pressed: bool) {
    let (bg, border, fg) = if pressed {
        (CYAN_WASH, CYAN, CYAN)
    } else {
        (RAISED, EDGE, MID)
    };
    panel(fb, r, pill(r.h), bg, border);
    text_centre(fb, r, label, fg, LABEL);
}

pub fn stamp(fb: &mut PaintBuffer, r: Rect, label: &str) {
    panel(fb, r, pill(r.h), VOID, EDGE);
    text_centre(fb, r, label, CYAN, LABEL);
}
