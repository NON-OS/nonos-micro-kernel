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

//! The ribbon's drawn marks: the pill chevron and the paragraph glyphs. They are
//! painted from rectangles rather than font glyphs so they stay crisp at any UI
//! scale and need no codepoint the bundled faces might not carry.

use nonos_app_skeleton::PaintBuffer;

const ICON_W: u32 = 16;
const ICON_H: u32 = 14;

pub(super) fn chevron(fb: &mut PaintBuffer, x: u32, cy: u32, argb: u32) {
    for i in 0..4u32 {
        fb.fill_rect(x + i, cy.saturating_sub(2) + i, 1, 1, argb);
        fb.fill_rect(x + 6 - i, cy.saturating_sub(2) + i, 1, 1, argb);
    }
}

pub(super) fn icon(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, kind: usize, argb: u32) {
    let ix = x + w.saturating_sub(ICON_W) / 2;
    let iy = y + h.saturating_sub(ICON_H) / 2;
    if kind == 5 {
        grid(fb, ix, iy, argb);
        return;
    }
    for r in 0..4u32 {
        let (bx, bw) = bar(kind, r, ix);
        fb.fill_rect(bx, iy + r * 4, bw, 2, argb);
        if kind >= 3 {
            fb.fill_rect(ix, iy + r * 4, 2, 2, argb);
        }
    }
}

fn bar(kind: usize, row: u32, ix: u32) -> (u32, u32) {
    if kind >= 3 {
        return (ix + 5, ICON_W - 5);
    }
    let bw = if row % 2 == 0 { ICON_W } else { ICON_W * 2 / 3 };
    match kind {
        1 => (ix + (ICON_W - bw) / 2, bw),
        2 => (ix + (ICON_W - bw), bw),
        _ => (ix, bw),
    }
}

fn grid(fb: &mut PaintBuffer, x: u32, y: u32, argb: u32) {
    for r in 0..4u32 {
        fb.fill_rect(x, y + r * 4, ICON_W, 1, argb);
    }
    for c in 0..3u32 {
        fb.fill_rect(x + c * (ICON_W / 2), y, 1, ICON_H - 1, argb);
    }
}
