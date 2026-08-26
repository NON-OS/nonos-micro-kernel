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
use crate::pm::theme::{CARD_BORDER, MUTED, TABLE_BG, TRACK_BG};

use super::chrome::Rect;
use super::metrics::{ROW_H, SCROLL_MIN_H, SCROLL_PAD, SCROLL_W, TBL_HEAD_H, TBL_RADIUS};
use super::table_geom::{self, Col};
use super::{table_head, table_row};

// The card border is stroked last so neither the header band nor a selected
// row's wash paints over it. Every offset inside the card comes from table_geom,
// which is also what the hit test reads, so a click cannot land on a row or a
// column other than the one drawn under it.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect, cols: &[Col]) {
    fb.fill_round(r.x, r.y, r.w, r.h, TBL_RADIUS, TABLE_BG);
    table_head::paint(fb, r, cols, state.sort);
    let rows = state.filtered();
    let visible = table_geom::visible_rows(r.h);
    for slot in 0..visible {
        match rows.get(state.scroll + slot) {
            Some(row) if table_geom::row_y(slot) + ROW_H <= r.h => {
                table_row::paint(state, fb, r, cols, row, slot)
            }
            _ => break,
        }
    }
    scrollbar(fb, r, state.scroll, rows.len(), visible);
    fb.stroke_round(r.x, r.y, r.w, r.h, TBL_RADIUS, 1, CARD_BORDER);
}

// Drawn only when the list is taller than the card: the thumb's height is the
// visible share of the whole list and its travel is the real scroll offset, so
// the bar reports the position rather than decorating the edge.
fn scrollbar(fb: &mut PaintBuffer, r: &Rect, scroll: usize, total: usize, visible: usize) {
    if total <= visible || r.h <= TBL_HEAD_H {
        return;
    }
    let top = r.y + TBL_HEAD_H;
    let track_h = r.h - TBL_HEAD_H;
    let x = r.x + r.w.saturating_sub(SCROLL_PAD + SCROLL_W);
    fb.fill_rect(x, top, SCROLL_W, track_h, TRACK_BG);
    let thumb = ((visible as u32 * track_h) / total as u32).max(SCROLL_MIN_H).min(track_h);
    let travel = (scroll as u32 * (track_h - thumb)) / (total - visible) as u32;
    fb.fill_rect(x, top + travel, SCROLL_W, thumb, MUTED);
}
