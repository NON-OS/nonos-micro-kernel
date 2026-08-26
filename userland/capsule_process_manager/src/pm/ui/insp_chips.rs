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

use crate::pm::format::CAP_TABLE;
use crate::pm::theme::{PILL_BG, PILL_BORDER};

use super::metrics::{BODY_PX, CHIP_GAP, CHIP_H, CHIP_PAD_X, CHIP_RADIUS};
use super::text;
use super::tint::cap_tint;

// The decoded grant list, wrapped by measured width rather than by a glyph
// count, because the body face is proportional. This is the surface with the
// room the table's Authority column never had.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, caps: u64) -> u32 {
    let mut cx = x;
    let mut cy = y;
    for &(bit, label) in CAP_TABLE {
        if caps & bit == 0 {
            continue;
        }
        let cw = text::width(fb, label, BODY_PX) + CHIP_PAD_X * 2;
        if cx > x && cx + cw > x + w {
            cx = x;
            cy += CHIP_H + CHIP_GAP;
        }
        fb.fill_round(cx, cy, cw, CHIP_H, CHIP_RADIUS, PILL_BG);
        fb.stroke_round(cx, cy, cw, CHIP_H, CHIP_RADIUS, 1, PILL_BORDER);
        let top = text::centred_top(cy, CHIP_H, BODY_PX);
        text::left(fb, cx + CHIP_PAD_X, top, label, cap_tint(bit), BODY_PX);
        cx += cw + CHIP_GAP;
    }
    cy + CHIP_H
}
