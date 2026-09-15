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

use super::icon_draw::draw;
use super::icon_path::Icon;
use super::list_cols::{cols, HEAD_H, HEAD_PX};
use super::measure_text::width_of;
use super::state::{SortMode, State};
use super::theme::{CY, HAIR, INK3};

const CARET: u32 = 10;
const CARET_GAP: u32 = 5;
const TEXT_DY: u32 = 4;

/// Which of the four columns the live sort mode belongs to. The sort model
/// cycles modes rather than directions, so the caret marks the active column
/// and says nothing about ascending or descending.
pub fn active_col(mode: SortMode) -> usize {
    match mode {
        SortMode::Name => 0,
        SortMode::Type => 1,
        SortMode::Size => 2,
        SortMode::Date => 3,
    }
}

/// The column header strip. Name is left-aligned over the name column and the
/// three meta columns right-align over the cells beneath them, so a label and
/// its values share one edge. Placement comes from `list_cols::cols`, the same
/// layout `list_click` tests a header click against.
pub fn paint_head(state: &State, fb: &mut PaintBuffer, left: u32, cw: u32, y: u32) {
    let c = cols(left, cw);
    let on = active_col(state.sort_mode);
    let ty = y + TEXT_DY;
    for (i, col) in c.cols.iter().enumerate() {
        let lit = i == on;
        let ink = if lit { CY } else { INK3 };
        let tw = width_of(fb, col.label, HEAD_PX);
        let tx = if i == 0 { col.x } else { (col.x + col.w).saturating_sub(tw) };
        let _ = fb.text_ttf(tx as i32, ty as i32, col.label, ink, HEAD_PX);
        if lit {
            let cx = if i == 0 {
                tx + tw + CARET_GAP
            } else {
                tx.saturating_sub(CARET + CARET_GAP)
            };
            draw(fb, Icon::Chevron, cx, ty + TEXT_DY, CARET, CY);
        }
    }
    fb.blend_rect(left, y + HEAD_H - 1, cw, 1, HAIR);
}
