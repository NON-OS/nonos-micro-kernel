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

//! Draw the page-break rule on the sheet. A break block lays out an empty
//! line, so without this the only sign of one would be the page after it.

use nonos_app_skeleton::PaintBuffer;

use super::canvas::{page_index, sheet_origin};
use super::layout::CHROME_PX;
use super::mode::Mode;
use super::state::State;
use super::theme;
use crate::doc::kind::BlockKind;

const MARK_LABEL: &str = "Page Break";
const MARK_GAP: u32 = 12;

pub(super) fn paint_page_breaks(state: &State, fb: &mut PaintBuffer) {
    if state.mode != Mode::Document {
        return;
    }
    let Some(page) = state.pages.get(page_index(state)) else {
        return;
    };
    let (sx, sy) = sheet_origin(state);
    let m = state.page_metrics.margin.max(0.0) as u32;
    let cw = state.page_metrics.content_width().max(0.0) as u32;
    let tw = fb.measure_ttf(MARK_LABEL, CHROME_PX).max(0) as u32;
    let seg = cw.saturating_sub(tw + MARK_GAP * 2) / 2;
    let th = theme::active();
    for line in &page.lines {
        let Some(block) = state.doc.blocks.get(line.block) else {
            continue;
        };
        if block.kind != BlockKind::PageBreak {
            continue;
        }
        let top = sy + m + line.y.max(0.0) as u32;
        let mid = top + (line.height.max(1.0) as u32) / 2;
        fb.blend_rect(sx + m, mid, seg, 1, th.line);
        fb.blend_rect(sx + m + cw.saturating_sub(seg), mid, seg, 1, th.line);
        let tx = (sx + m + cw.saturating_sub(tw) / 2) as i32;
        let _ = fb.text_ttf(tx, top as i32, MARK_LABEL, th.muted, CHROME_PX);
    }
}
