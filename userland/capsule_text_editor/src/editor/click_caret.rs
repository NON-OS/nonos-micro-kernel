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

//! Place the caret where the pointer clicked in the text body.

use super::byte_at::byte_at;
use super::canvas::{page_index, sheet_origin};
use super::layout::{line_height, text_left, PAD_TOP};
use super::mode::Mode;
use super::state::State;
use crate::doc::hit::caret_at;
use crate::doc::ttf_measure::TtfMeasurer;

pub(super) fn click_caret(state: &mut State, x: i32, y: i32) {
    if x < 0 || y < 0 {
        return;
    }
    if state.mode == Mode::Document {
        let (sx, sy) = sheet_origin(state);
        let m = state.page_metrics.margin;
        let px = (x - sx as i32) as f32 - m;
        let py = (y - sy as i32) as f32 - m;
        if let Some(page) = state.pages.get(page_index(state)) {
            let (block, off) = caret_at(page, &state.doc, px, py, &TtfMeasurer);
            state.caret = state.doc_byte(block, off);
        }
        return;
    }
    let (x, y) = (x as u32, y as u32);
    let top = state.pane_y + PAD_TOP;
    if y < top {
        state.caret = 0;
        return;
    }
    let row = (y - top) / line_height(state.font_scale);
    let line = state.scroll_line + row;
    // Same measured cell width and pane origin the body was drawn with, so
    // clicks land on the character under the pointer.
    let col = x.saturating_sub(text_left(state.pane_x)) / state.glyph_advance.max(1);
    state.caret = byte_at(&state.buf[..state.len], state.wrap_cols, line, col);
}
