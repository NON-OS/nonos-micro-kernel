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

use crate::pm::state::Row;
use crate::pm::theme::{DANGER, RULE};

use super::super::chrome::Rect;
use super::super::matrix_geom::{self as geom, MATRIX};
use super::super::tint::cap_tint;

const DOT_RADIUS: u32 = 3;
const RING_PAD: u32 = 2;
const TICK_H: u32 = 1;

// A held authority is a filled dot in its risk-class colour; an unheld one leaves
// a hairline tick, so an empty column still reads as a column rather than as a
// gap. The ring is the finding this screen exists to surface: a sensitive grant
// on a process the kernel does not protect. cap_tint, DANGER and RULE are all
// opaque, and fill_round strokes blend anyway, so nothing punches the row.
pub fn paint(fb: &mut PaintBuffer, r: &Rect, row: &Row, y: u32, protected: bool) {
    let cw = geom::cell_w(r.w);
    let dot = geom::CELL_DOT;
    for (col, (bit, _)) in MATRIX.iter().enumerate() {
        let cx = r.x + geom::cell_x(r.w, col) + cw.saturating_sub(dot) / 2;
        let cy = y + (geom::ROW_H - dot) / 2;
        if row.caps & bit == 0 {
            fb.fill_rect(cx, cy + dot / 2, dot, TICK_H, RULE);
            continue;
        }
        fb.fill_round(cx, cy, dot, dot, DOT_RADIUS, cap_tint(*bit));
        if !protected {
            let ring = dot + RING_PAD * 2;
            let (rx, ry) = (cx.saturating_sub(RING_PAD), cy.saturating_sub(RING_PAD));
            fb.stroke_round(rx, ry, ring, ring, DOT_RADIUS + RING_PAD, 1, DANGER);
        }
    }
}
