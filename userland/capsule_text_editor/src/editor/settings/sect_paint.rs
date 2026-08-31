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

//! Painter for the six table-driven sections. Rows come from the section table
//! and every control rect is built by `control_box`, the same function the
//! router calls, so a switch is never drawn where a press would miss it.

use nonos_app_skeleton::PaintBuffer;

use super::card::{
    card_rect, control_box, row_y, DROP_H, DROP_MIN_W, RADIUS, ROW_H, ROW_PAD, TOGGLE_H, TOGGLE_W,
};
use super::geom::{head_top, lh, pane_x, HEAD_PX, PANE_PAD, ROW_PX};
use super::sect::{sect_rect, Ctl, Section};
use super::sect_state::sect_on;
use super::style::{drop_dim, CARD_BG, HAIRLINE, SWITCH, TEXT};
use crate::editor::widget::{dropdown_w, paint_dropdown, paint_toggle, truncate_to_width};

pub(super) fn paint_section(fb: &mut PaintBuffer, nav: usize, sec: &Section) {
    let width = fb.width;
    let hx = (pane_x() + PANE_PAD) as i32;
    let _ = fb.text_ttf(hx, head_top() as i32, sec.head, TEXT, HEAD_PX);
    let (cx, cy, cw, ch) = sect_rect(width, sec.rows.len());
    fb.panel(cx, cy, cw, ch, RADIUS, CARD_BG, HAIRLINE);
    for i in 1..sec.rows.len() {
        fb.fill_rect(cx + 1, cy + i as u32 * ROW_H, cw.saturating_sub(2), 1, HAIRLINE);
    }
    for (i, row) in sec.rows.iter().enumerate() {
        paint_row(fb, width, nav, i, row);
    }
}

fn paint_row(fb: &mut PaintBuffer, width: u32, nav: usize, row: usize, spec: &(&str, Ctl)) {
    let (cx, _, cw, _) = card_rect(width);
    let ty = (row_y(width, row) + ROW_H.saturating_sub(lh(ROW_PX)) / 2) as i32;
    let avail = cw.saturating_sub(ROW_PAD * 2 + DROP_MIN_W + 16) as i32;
    let cut = truncate_to_width(fb, spec.0, ROW_PX, avail);
    let _ = fb.text_ttf((cx + ROW_PAD) as i32, ty, cut, TEXT, ROW_PX);
    match spec.1 {
        Ctl::Toggle(bit) => {
            let rect = control_box(width, row, TOGGLE_W, TOGGLE_H);
            paint_toggle(fb, rect, sect_on(nav, bit), SWITCH);
        }
        Ctl::Drop(value) => {
            let style = drop_dim();
            let w = dropdown_w(fb, value, ROW_PX, DROP_MIN_W, &style);
            paint_dropdown(fb, control_box(width, row, w, DROP_H), value, ROW_PX, &style);
        }
    }
}
