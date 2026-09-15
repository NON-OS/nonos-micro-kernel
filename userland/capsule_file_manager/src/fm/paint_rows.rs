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

//! The detail list: one rounded row per entry, placed by `row_geom::row_slots`
//! so a click hit-tests against exactly what was drawn.

use nonos_app_skeleton::PaintBuffer;

use super::layout::{content_w, content_x, ROW_H};
use super::list_head::paint_head;
use super::paint_row::row_body;
use super::row_geom::{list_top, row_slots};
use super::screen_row::empty_state;
use super::state::State;
use super::theme::SELECT_BG;

pub fn paint_rows(state: &State, fb: &mut PaintBuffer) {
    let left = content_x();
    let cw = content_w(fb.width);
    paint_head(state, fb, left, cw, state.row_top);
    if state.entries.is_empty() {
        let note = if state.filter.is_empty() {
            "This folder has nothing in it."
        } else {
            "No entry matches the current filter."
        };
        empty_state(fb, left, list_top(state) + 40, cw, "Nothing here", note);
        return;
    }
    for slot in row_slots(state) {
        let entry = &state.entries[slot.index];
        let lit = slot.index == state.cursor
            || state.selected.iter().any(|path| path == &entry.full_path);
        if lit {
            fb.fill_round(left, slot.y + 2, cw, ROW_H - 4, 10, SELECT_BG);
        }
        row_body(state, fb, slot.index, slot.y, left, cw);
    }
}
