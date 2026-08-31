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

use crate::pm::state::Sort;
use crate::pm::theme::{ACCENT, HEADER_BG, MUTED, RULE};

use super::chrome::Rect;
use super::metrics::{BODY_PX, SORT_MARK_GAP, SORT_MARK_H, SORT_MARK_W, TBL_HEAD_H, TBL_RADIUS};
use super::table_geom::{self, Col};
use super::text;

fn label(col: Col) -> &'static [u8] {
    match col {
        Col::Name => b"NAME",
        Col::Pid => b"PID",
        Col::State => b"STATE",
        Col::Cpu => b"CPU",
        Col::Mem => b"MEMORY",
        Col::Uptime => b"UPTIME",
        Col::Auth => b"AUTHORITY",
    }
}

// The band shares the card's top radius, so its square bottom is painted back in
// rather than left as two notches over the table ground. Only a column that
// maps to a sort takes the accent, and refresh() orders cpu and memory largest
// first while name and pid run smallest first.
pub fn paint(fb: &mut PaintBuffer, r: &Rect, cols: &[Col], sort: Sort) {
    fb.fill_round(r.x, r.y, r.w, TBL_HEAD_H, TBL_RADIUS, HEADER_BG);
    fb.fill_rect(r.x, r.y + TBL_HEAD_H - TBL_RADIUS, r.w, TBL_RADIUS, HEADER_BG);
    fb.fill_rect(r.x, r.y + TBL_HEAD_H, r.w, 1, RULE);
    let top = text::centred_top(r.y, TBL_HEAD_H, BODY_PX);
    for col in cols {
        let active = table_geom::sort_for(*col) == Some(sort);
        let tint = if active { ACCENT } else { MUTED };
        let x = r.x + table_geom::col_x(cols, r.w, *col);
        let after = text::left(fb, x, top, label(*col), tint, BODY_PX).max(0) as u32;
        if active {
            mark(fb, after + SORT_MARK_GAP, r.y, matches!(sort, Sort::Name | Sort::Pid));
        }
    }
}

// NotoSans carries no triangle glyph, so the direction mark is drawn rather than
// typed. It is still placed from the label's measured advance, never from a
// glyph count.
fn mark(fb: &mut PaintBuffer, x: u32, band_y: u32, up: bool) {
    let y = band_y + (TBL_HEAD_H - SORT_MARK_H) / 2;
    for i in 0..SORT_MARK_H {
        let step = if up { SORT_MARK_H - 1 - i } else { i };
        let w = SORT_MARK_W.saturating_sub(step * 2);
        if w > 0 {
            fb.fill_rect(x + step, y + i, w, 1, ACCENT);
        }
    }
}
