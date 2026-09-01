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

use super::fit_text::{fit_text, width_of};
use super::tab_pill::{LABEL_PX, RADIUS};
use super::tokens::TAB_HOVER;
use crate::layout::Rect;

/// A compact labelled button inside the titlebar accessory: a soft rounded
/// ground with its label measured and centred, never counted in glyphs.
pub fn draw_chip(fb: &mut PaintBuffer, r: Rect, label: &str, fg: u32) {
    if r.w == 0 {
        return;
    }
    fb.fill_round(r.x, r.y, r.w, r.h, RADIUS, TAB_HOVER);
    let cut = fit_text(fb, label, LABEL_PX, r.w.saturating_sub(4));
    let x = r.x + r.w / 2 - width_of(fb, cut, LABEL_PX) / 2;
    let y = (r.y + r.h / 2) as i32 - (LABEL_PX as i32) / 2;
    fb.text_ttf(x as i32, y, cut, fg, LABEL_PX);
}
