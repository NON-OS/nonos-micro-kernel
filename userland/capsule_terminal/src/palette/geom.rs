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

use crate::layout::Rect;

/// Result rows the overlay will show. Beyond this the reader is better served
/// by typing another character than by scrolling a list they cannot see.
pub const MAX_ROWS: usize = 8;

const MARGIN: u32 = 24;
const PAD: u32 = 10;

/// The panel: centred over `body`, tall enough for the query row plus `rows`
/// results, and never taller than the band it sits in.
pub fn panel(body: Rect, row_h: u32, rows: u32) -> Rect {
    let w = (body.w * 3 / 4).min(body.w.saturating_sub(MARGIN.min(body.w / 2) * 2));
    let want = PAD * 2 + row_h * (rows + 1);
    let h = want.min(body.h);
    let top = MARGIN.min(body.h.saturating_sub(h));
    Rect { x: body.x + body.w.saturating_sub(w) / 2, y: body.y + top, w, h }
}

pub fn query_row(p: Rect, row_h: u32) -> Rect {
    Rect { x: p.x + PAD, y: p.y + PAD, w: p.w.saturating_sub(PAD * 2), h: row_h }
}

pub fn row(p: Rect, i: u32, row_h: u32) -> Rect {
    let q = query_row(p, row_h);
    Rect { x: q.x, y: q.y + row_h * (i + 1), w: q.w, h: row_h }
}

/// Rows the panel can show whole, so the painter and the selection clamp agree
/// on how much of the match list is reachable.
pub fn rows_fit(p: Rect, row_h: u32) -> u32 {
    let usable = p.h.saturating_sub(PAD * 2 + row_h);
    (usable / row_h.max(1)).min(MAX_ROWS as u32)
}
