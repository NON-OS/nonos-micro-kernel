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

use crate::about::theme::{DANGER, MUTED, OK, OK_TINT, PILL_BG, PILL_BORDER};

use super::metrics::{BODY_PX, CHIP_DOT, CHIP_DOT_GAP, CHIP_H, CHIP_PAD_X, CHIP_RADIUS};
use super::text;

// Width the chip will occupy, without a surface. The wrapper needs it before it
// commits a row, and the click path has no PaintBuffer, so both come through
// the measure-only path rather than the painted one.
pub fn width_of(label: &[u8]) -> u32 {
    CHIP_PAD_X * 2 + CHIP_DOT + CHIP_DOT_GAP + text::width_of(label, BODY_PX)
}

// A granted/denied pill. The dot carries the state and the label carries the
// name, so the two never depend on colour alone to be told apart.
pub fn chip(fb: &mut PaintBuffer, x: u32, y: u32, label: &[u8], on: bool) -> u32 {
    let w = width_of(label);
    let bg = if on { OK_TINT } else { PILL_BG };
    fb.fill_round(x, y, w, CHIP_H, CHIP_RADIUS, bg);
    fb.stroke_round(x, y, w, CHIP_H, CHIP_RADIUS, 1, PILL_BORDER);
    let dot_x = x + CHIP_PAD_X + CHIP_DOT / 2;
    fb.circle(dot_x, y + CHIP_H / 2, CHIP_DOT / 2, if on { OK } else { DANGER });
    let top = text::top_of(y as i32, CHIP_H, BODY_PX);
    let fg = if on { OK } else { MUTED };
    text::line(fb, x + CHIP_PAD_X + CHIP_DOT + CHIP_DOT_GAP, top, label, fg, BODY_PX);
    w
}
