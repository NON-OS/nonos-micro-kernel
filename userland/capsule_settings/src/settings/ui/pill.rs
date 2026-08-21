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

use crate::settings::schema::rows::Tone;

use super::metrics::{BODY_PX, PILL_H, PILL_PAD_X};
use super::text;
use super::theme::{IDLE, OK, PILL_BG, PILL_BORDER, VALUE_FG, WARN};

pub fn tone_argb(tone: Tone) -> u32 {
    match tone {
        Tone::Ok => OK,
        Tone::Warn => WARN,
        Tone::Idle => IDLE,
    }
}

/// Draws the badge right-aligned to `right_x` and returns its left edge.
pub fn draw(fb: &mut PaintBuffer, right_x: u32, y: u32, label: &str, tone: Tone) -> u32 {
    let dot = 7u32;
    let text_w = text::width(fb, label, BODY_PX);
    let w = PILL_PAD_X * 2 + dot + 8 + text_w;
    let x = right_x.saturating_sub(w);
    fb.fill_round(x, y, w, PILL_H, PILL_H / 2, PILL_BG);
    fb.stroke_round(x, y, w, PILL_H, PILL_H / 2, 1, PILL_BORDER);
    let cy = y + PILL_H / 2;
    fb.circle(x + PILL_PAD_X + dot / 2, cy, dot / 2, tone_argb(tone));
    let top = text::centred_top(y, PILL_H, BODY_PX);
    text::left(fb, x + PILL_PAD_X + dot + 8, top, label, VALUE_FG, BODY_PX);
    x
}
