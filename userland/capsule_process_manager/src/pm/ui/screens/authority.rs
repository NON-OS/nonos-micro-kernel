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

use crate::pm::state::State;
use crate::pm::theme::{CARD_BORDER, HEADER_BG, MUTED, RULE, TABLE_BG};

use super::super::chrome::Rect;
use super::super::matrix_geom::{self as geom, MATRIX};
use super::super::metrics::{BODY_PX, TBL_RADIUS};
use super::super::text;
use super::super::tint::cap_tint;
use super::{auth_legend, auth_row};

// The screen the hex `caps` dump could never be: one row per process, one column
// per sensitive authority, so an unexpected grant is a shape rather than a number
// nobody decodes. Every rect on it comes out of matrix_geom, which is also what
// the hit test reads, so a click cannot land on a cell other than the drawn one.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    fb.fill_round(r.x, r.y, r.w, r.h, TBL_RADIUS, TABLE_BG);
    head(fb, r);
    let rows = state.filtered();
    for slot in 0..geom::visible_rows(r.h) {
        match rows.get(state.scroll + slot) {
            Some(row) => auth_row::paint(state, fb, r, row, slot),
            None => break,
        }
    }
    auth_legend::paint(fb, r);
    fb.stroke_round(r.x, r.y, r.w, r.h, TBL_RADIUS, 1, CARD_BORDER);
}

// The header band is rounded at the top and squared at the bottom so it meets the
// first row flush. Each abbreviation takes its risk-class colour, which is what
// ties a column to the legend without spelling the name out twice.
fn head(fb: &mut PaintBuffer, r: &Rect) {
    fb.fill_round(r.x, r.y, r.w, geom::HEAD_H, TBL_RADIUS, HEADER_BG);
    fb.fill_rect(r.x, r.y + geom::HEAD_H - TBL_RADIUS, r.w, TBL_RADIUS, HEADER_BG);
    fb.fill_rect(r.x, r.y + geom::HEAD_H, r.w, 1, RULE);
    let top = text::centred_top(r.y, geom::HEAD_H, BODY_PX);
    text::left(fb, r.x + geom::PAD_X, top, b"PROCESS", MUTED, BODY_PX);
    let cw = geom::cell_w(r.w);
    for (col, (bit, label)) in MATRIX.iter().enumerate() {
        let pad = cw.saturating_sub(text::width(fb, label, BODY_PX)) / 2;
        text::left(fb, r.x + geom::cell_x(r.w, col) + pad, top, label, cap_tint(*bit), BODY_PX);
    }
}
