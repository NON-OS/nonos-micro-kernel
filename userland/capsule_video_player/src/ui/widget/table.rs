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

use crate::ui::fit::fit;
use crate::ui::layout::Rect;
use crate::ui::paint::{rrect, shape};
use crate::ui::text::{center_y, BODY_PX};
use crate::ui::theme;

pub const HEAD_H: u32 = 32;
pub const ROW_H: u32 = 46;
pub const PAD: u32 = 14;

pub struct Column {
    pub title: &'static str,
    pub weight: u32,
    pub min: u32,
}

pub fn column_x(cols: &[Column], w: u32, index: usize) -> (u32, u32) {
    let total: u32 = cols.iter().map(|c| c.weight).sum::<u32>().max(1);
    let mut pen = 0;
    for (i, col) in cols.iter().enumerate() {
        let span = (w as u64 * col.weight as u64 / total as u64) as u32;
        let span = span.max(col.min);
        if i == index {
            return (pen, span.min(w.saturating_sub(pen)));
        }
        pen += span;
    }
    (w, 0)
}

pub fn row_rect(body: Rect, slot: usize) -> Rect {
    Rect { x: body.x, y: body.y + slot as u32 * ROW_H, w: body.w, h: ROW_H }
}

pub fn rows_visible(h: u32) -> usize {
    (h / ROW_H) as usize
}

pub fn paint_head(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, cols: &[Column]) {
    for (i, col) in cols.iter().enumerate() {
        let (off, span) = column_x(cols, w, i);
        let text = fit(col.title, span.saturating_sub(PAD), BODY_PX);
        fb.text_ttf((x + off) as i32, center_y(y, HEAD_H), text, theme::LABEL, BODY_PX);
    }
    shape::hline(fb, x, y + HEAD_H, w, theme::BORDER);
}

pub fn paint_row_bg(fb: &mut PaintBuffer, r: Rect, selected: bool) {
    if selected {
        rrect::fill_round(fb, r.x, r.y, r.w, r.h, 8, theme::SELECT);
        fb.fill_rect(r.x, r.y, 3, r.h, theme::ACCENT);
        return;
    }
    shape::hline(fb, r.x, r.y + r.h - 1, r.w, theme::BORDER_SOFT);
}

pub fn paint_cell(fb: &mut PaintBuffer, r: Rect, cols: &[Column], i: usize, text: &str, ink: u32) {
    let (off, span) = column_x(cols, r.w, i);
    let text = fit(text, span.saturating_sub(PAD), BODY_PX);
    fb.text_ttf((r.x + off + PAD) as i32, center_y(r.y, r.h), text, ink, BODY_PX);
}
