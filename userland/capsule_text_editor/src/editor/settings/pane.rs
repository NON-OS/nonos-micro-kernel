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

//! The General panel: a section heading over one card of seven labelled rows.
//! The three dropdowns have no popup and no backing setting, so they are drawn
//! from the dimmed style; only the four switches are live.

use nonos_app_skeleton::PaintBuffer;

use super::card::{
    card_rect, control_box, row_y, DROP_H, DROP_MIN_W, DROP_VALUES, RADIUS, ROWS, ROW_H, ROW_LABELS,
    ROW_PAD, TOGGLE_H, TOGGLE_W,
};
use super::geom::{head_top, lh, pane_x, HEAD_PX, PANE_PAD, ROW_PX};
use super::state::SettingsState;
use super::style::{drop_dim, CARD_BG, HAIRLINE, SWITCH, TEXT};
use crate::editor::widget::{dropdown_w, paint_dropdown, paint_toggle, truncate_to_width};

pub(super) fn paint_pane(fb: &mut PaintBuffer, st: &SettingsState) {
    let width = fb.width;
    let hx = (pane_x() + PANE_PAD) as i32;
    let _ = fb.text_ttf(hx, head_top() as i32, "General", TEXT, HEAD_PX);
    let (cx, cy, cw, ch) = card_rect(width);
    fb.panel(cx, cy, cw, ch, RADIUS, CARD_BG, HAIRLINE);
    for i in 1..ROWS {
        fb.fill_rect(cx + 1, cy + i as u32 * ROW_H, cw.saturating_sub(2), 1, HAIRLINE);
    }
    for i in 0..ROWS {
        paint_row(fb, width, i, st);
    }
}

fn paint_row(fb: &mut PaintBuffer, width: u32, row: usize, st: &SettingsState) {
    let (cx, _, cw, _) = card_rect(width);
    let ty = (row_y(width, row) + ROW_H.saturating_sub(lh(ROW_PX)) / 2) as i32;
    let avail = cw.saturating_sub(ROW_PAD * 2 + DROP_MIN_W + 16) as i32;
    let cut = truncate_to_width(fb, ROW_LABELS[row], ROW_PX, avail);
    let _ = fb.text_ttf((cx + ROW_PAD) as i32, ty, cut, TEXT, ROW_PX);
    if row < DROP_VALUES.len() {
        let style = drop_dim();
        let value = DROP_VALUES[row];
        let w = dropdown_w(fb, value, ROW_PX, DROP_MIN_W, &style);
        let rect = control_box(width, row, w, DROP_H);
        paint_dropdown(fb, rect, value, ROW_PX, &style);
    } else {
        let rect = control_box(width, row, TOGGLE_W, TOGGLE_H);
        paint_toggle(fb, rect, st.switches[row - DROP_VALUES.len()], SWITCH);
    }
}
