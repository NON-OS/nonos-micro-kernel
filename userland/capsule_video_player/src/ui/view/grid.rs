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

use nonos_app_skeleton::paint::PaintBuffer;

use crate::app::browse::Browse;
use crate::ui::format::{duration, resolution, size};
use crate::ui::layout::Rect;
use crate::ui::theme;
use crate::ui::widget::card::{columns, paint_tile, tile_rect, TILE_H};
use crate::ui::widget::scrollbar::{paint_scrollbar, track};
use crate::ui::widget::table::{paint_cell, paint_head, paint_row_bg, row_rect, rows_visible, Column, HEAD_H};

pub const COLUMNS: [Column; 5] = [
    Column { title: "NAME", weight: 44, min: 160 },
    Column { title: "DURATION", weight: 14, min: 92 },
    Column { title: "RESOLUTION", weight: 16, min: 104 },
    Column { title: "SIZE", weight: 13, min: 82 },
    Column { title: "KIND", weight: 13, min: 70 },
];

pub fn grid_capacity(body: Rect) -> usize {
    let rows = ((body.h + 16) / (TILE_H + 16)).max(1) as usize;
    rows * columns(body.w)
}

pub fn paint_grid(fb: &mut PaintBuffer, browse: &Browse, body: Rect) {
    let cap = grid_capacity(body);
    for slot in 0..browse.len().saturating_sub(browse.scroll).min(cap) {
        let index = browse.scroll + slot;
        let Some(item) = browse.get(index) else { break };
        let r = tile_rect(body, slot);
        if r.y + r.h > body.y + body.h {
            break;
        }
        paint_tile(fb, r, item, index == browse.sel);
    }
}

pub fn paint_list(fb: &mut PaintBuffer, browse: &Browse, body: Rect) {
    let w = body.w.saturating_sub(12);
    paint_head(fb, body.x, body.y, w, &COLUMNS);
    let rows = Rect { x: body.x, y: body.y + HEAD_H + 4, w, h: body.h.saturating_sub(HEAD_H + 4) };
    let visible = rows_visible(rows.h);
    for slot in 0..visible {
        let index = browse.scroll + slot;
        let Some(item) = browse.get(index) else { break };
        let r = row_rect(rows, slot);
        paint_row_bg(fb, r, index == browse.sel);
        let ink = if index == browse.sel { theme::TEXT } else { theme::TEXT_DIM };
        paint_cell(fb, r, &COLUMNS, 0, item.title(), theme::TEXT);
        paint_cell(fb, r, &COLUMNS, 1, &duration(item.duration_ms), ink);
        paint_cell(fb, r, &COLUMNS, 2, &resolution(item.width, item.height), ink);
        paint_cell(fb, r, &COLUMNS, 3, &size(item.size), ink);
        paint_cell(fb, r, &COLUMNS, 4, item.kind.label(), ink);
    }
    paint_scrollbar(fb, track(body), browse.scroll, visible, browse.len());
}
