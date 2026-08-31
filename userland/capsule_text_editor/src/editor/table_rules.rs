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

//! The grid itself. Rules are blended, never filled: they land on pixels the
//! sheet already painted, and a raw write would punch the sheet colour out from
//! under them.

use nonos_app_skeleton::PaintBuffer;

use super::theme;
use crate::doc::table::geom::col_x;

fn hline(fb: &mut PaintBuffer, x: i32, y: i32, w: f32, argb: u32) {
    if x < 0 || y < 0 || w <= 0.0 {
        return;
    }
    fb.blend_rect(x as u32, y as u32, w as u32, 1, argb);
}

fn vline(fb: &mut PaintBuffer, x: i32, y: i32, h: u32, argb: u32) {
    if x < 0 || y < 0 || h == 0 {
        return;
    }
    fb.blend_rect(x as u32, y as u32, 1, h, argb);
}

pub(super) fn paint_rules(fb: &mut PaintBuffer, x: i32, y: i32, w: &[f32], h: u32, last: bool) {
    let argb = theme::active().line;
    let total = col_x(w, w.len());
    hline(fb, x, y, total, argb);
    if last {
        hline(fb, x, y + h.saturating_sub(1) as i32, total, argb);
    }
    for i in 0..=w.len() {
        vline(fb, x + col_x(w, i) as i32, y, h, argb);
    }
}
