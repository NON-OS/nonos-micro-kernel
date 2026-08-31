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

//! The ribbon row. Painted after the tab strip and the menu bar so it owns the
//! band between them and the pane, and it returns the cells it drew for the
//! hit-test to reuse.

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

use super::cell::paint_cell;
use super::cells::cells;
use super::metrics::{band_top, cell_h, cell_top, text_top, Geom, RibbonCell};
use super::snapshot::RibbonState;
use crate::editor::layout::{ACTIVITY_W, RIBBON_H};
use crate::editor::theme;

pub(in crate::editor) fn paint_ribbon(
    fb: &mut PaintBuffer,
    width: u32,
    st: &RibbonState,
    open: Option<usize>,
) -> Vec<RibbonCell> {
    let th = theme::active();
    let (top, band) = (band_top(), RIBBON_H.saturating_sub(1));
    let span = width.saturating_sub(ACTIVITY_W);
    fb.fill_rect(ACTIVITY_W, top, span, band, th.header_bg);
    fb.fill_rect(ACTIVITY_W, top + band, span, 1, th.line);

    let labels = [st.style.as_str(), st.font.as_str(), st.size.as_str()];
    let out = cells(&labels);
    let (cy, ch) = (cell_top(), cell_h());
    let geom = Geom { cy, ch, ty: (cy + text_top(ch)) as i32 };
    for c in out.iter() {
        if c.x1 > width {
            break;
        }
        paint_cell(fb, c, &geom, &labels, st, open);
    }
    out
}
