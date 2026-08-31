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
use nonos_toolkit::font::ttf::line_height;

use super::history_row;
use crate::calc::state::State;
use crate::calc::theme::{FAINT, LINE_2, PANEL};
use crate::calc::ui::history_geom::{capacity, pane, PAD};
use crate::calc::ui::metrics::{PX_BODY, R_PANEL};

const EMPTY: &str = "No calculations yet";

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let (x, y, w, h) = pane(fb.width as i32, fb.height as i32);
    if w <= PAD * 2 || h <= 0 || x < 0 || y < 0 {
        return;
    }
    fb.panel(x as u32, y as u32, w as u32, h as u32, R_PANEL as u32, PANEL, LINE_2);
    if state.history.len() == 0 {
        let lh = line_height(PX_BODY).max(1);
        let tx = x + (w - fb.measure_ttf(EMPTY, PX_BODY)) / 2;
        fb.text_ttf(tx, y + (h - lh) / 2, EMPTY, FAINT, PX_BODY);
        return;
    }
    let rows = capacity(fb.height as i32).min(state.history.len());
    for i in 0..rows {
        if let Some(entry) = state.history.get(i) {
            history_row::paint(state, fb, i, entry);
        }
    }
}
