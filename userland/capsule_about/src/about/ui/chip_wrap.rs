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

use crate::about::theme::{MUTED, OK};

use super::chip;
use super::metrics::{BODY_PX, CHIP_GAP, CHIP_H};
use super::text::line;

// The wrap the painter and the extent both walk. It measures with chip::width_of
// and never with a glyph count, so the height reported before a frame is drawn is
// the height that frame will occupy.
pub fn wrap_h(labels: &[&[u8]], w: u32) -> u32 {
    if labels.is_empty() {
        return 0;
    }
    let mut cx = 0u32;
    let mut rows = 1u32;
    for label in labels {
        let cw = chip::width_of(label);
        if cx > 0 && cx + cw > w {
            cx = 0;
            rows += 1;
        }
        cx += cw + CHIP_GAP;
    }
    rows * CHIP_H + (rows - 1) * CHIP_GAP
}

// The same flow, drawn. A chip straddling the pane's top edge keeps its label and
// loses its pill: the pill is a rounded fill that cannot be placed above zero, and
// half a pill reads worse than none.
pub fn wrap(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, labels: &[&[u8]], on: bool) -> u32 {
    let mut cx = x;
    let mut cy = y;
    for label in labels {
        let cw = chip::width_of(label);
        if cx > x && cx + cw > x + w {
            cx = x;
            cy += (CHIP_H + CHIP_GAP) as i32;
        }
        if cy >= 0 {
            chip::chip(fb, cx, cy as u32, label, on);
        } else if cy + CHIP_H as i32 > 0 {
            line(fb, cx, cy, label, if on { OK } else { MUTED }, BODY_PX);
        }
        cx += cw + CHIP_GAP;
    }
    wrap_h(labels, w)
}
