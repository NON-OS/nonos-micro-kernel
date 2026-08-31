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

use crate::ui::fit::fit;
use crate::ui::layout::Rect;
use crate::ui::paint::{rrect, shape};
use crate::ui::text::BODY_PX;
use crate::ui::theme;

pub const TRACK_W: u32 = 44;
pub const TRACK_H: u32 = 24;
const KNOB: u32 = 18;
const INSET: u32 = 3;

pub fn track(x: u32, y: u32) -> Rect {
    Rect { x, y, w: TRACK_W, h: TRACK_H }
}

pub fn knob_x(x: u32, on: bool) -> u32 {
    if on {
        x + TRACK_W.saturating_sub(KNOB + INSET)
    } else {
        x + INSET
    }
}

pub fn paint_toggle(fb: &mut PaintBuffer, x: u32, y: u32, on: bool) {
    let fill = if on { theme::ACCENT_DIM } else { theme::TRACK };
    rrect::fill_round(fb, x, y, TRACK_W, TRACK_H, TRACK_H / 2, fill);
    if !on {
        rrect::stroke_round(fb, x, y, TRACK_W, TRACK_H, TRACK_H / 2, 1, theme::BORDER);
    }
    let kx = knob_x(x, on);
    shape::circle(fb, kx + KNOB / 2, y + TRACK_H / 2, KNOB / 2, theme::TEXT);
}

pub fn paint_row(fb: &mut PaintBuffer, r: Rect, title: &str, note: &str, on: bool) {
    let room = r.w.saturating_sub(TRACK_W + 24);
    fb.text_ttf(r.x as i32, r.y as i32, fit(title, room, BODY_PX), theme::TEXT, BODY_PX);
    fb.text_ttf(r.x as i32, (r.y + 20) as i32, fit(note, room, BODY_PX), theme::TEXT_MUTED, BODY_PX);
    let tx = (r.x + r.w).saturating_sub(TRACK_W);
    paint_toggle(fb, tx, r.y + r.h.saturating_sub(TRACK_H) / 2, on);
}
