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

//! Paint the document canvas: a page sheet, then every line box of the current
//! page in the run style the model assigned it. Text goes through the unclamped
//! `_with` font entry points; the clamped ones raise anything below 17px, and
//! the paint would then disagree with the layout.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf::{builtin_face, draw_text_with};

use super::state::State;
use super::theme;
use crate::doc::hit::{caret_rect, line_for};
use crate::doc::style::Family;
use crate::doc::ttf_measure::TtfMeasurer;

const SHEET_R: u32 = 10;
const SHEET_TOP: u32 = 12;
const SHEET_BG: u32 = 0xFF14_1A22;
const SHEET_SHADOW: u32 = 0x5000_0000;

pub(super) fn page_index(state: &State) -> usize {
    let (block, off) = state.doc_pos(state.caret);
    let off = state.snap(block, off);
    state
        .pages
        .iter()
        .position(|p| line_for(p, block, off).is_some())
        .or_else(|| state.pages.iter().position(|p| p.lines.iter().any(|l| l.block == block)))
        .unwrap_or_else(|| state.pages.len().saturating_sub(1))
}

pub(super) fn sheet_origin(state: &State) -> (u32, u32) {
    let w = state.page_metrics.width as u32;
    (state.pane_x + state.pane_w.saturating_sub(w) / 2, state.pane_y + SHEET_TOP)
}

pub(super) fn paint_document(state: &mut State, fb: &mut PaintBuffer) {
    state.caret = state.caret.min(state.len);
    if state.pages.is_empty() {
        state.reflow();
    }
    fb.fill_rect(
        state.pane_x,
        state.pane_y,
        state.pane_w,
        state.pane_h,
        theme::active().background,
    );
    let (sx, sy) = sheet_origin(state);
    let sw = state.page_metrics.width as u32;
    let sh = (state.page_metrics.height as u32).min(state.pane_h.saturating_sub(2 * SHEET_TOP));
    fb.shadow_round(sx, sy, sw, sh, SHEET_R, 10, SHEET_SHADOW);
    fb.fill_round(sx, sy, sw, sh, SHEET_R, SHEET_BG);
    paint_lines(state, fb, sx, sy);
    paint_caret(state, fb, sx, sy);
}

fn paint_caret(state: &State, fb: &mut PaintBuffer, sx: u32, sy: u32) {
    let Some(page) = state.pages.get(page_index(state)) else { return };
    let (block, off) = state.doc_pos(state.caret);
    let off = state.snap(block, off);
    let Some((cx, cy, ch)) = caret_rect(page, &state.doc, block, off, &TtfMeasurer) else {
        return;
    };
    let m = state.page_metrics.margin as u32;
    let x = sx + m + cx.max(0.0) as u32;
    let y = sy + m + cy.max(0.0) as u32;
    fb.blend_rect(x, y, 2, ch.max(1.0) as u32, theme::active().caret);
}

fn paint_lines(state: &State, fb: &mut PaintBuffer, sx: u32, sy: u32) {
    let Some(page) = state.pages.get(page_index(state)) else { return };
    let m = state.page_metrics.margin as i32;
    let (w, h, stride) = (fb.width, fb.height, fb.stride_words as usize);
    for line in &page.lines {
        let Some(b) = state.doc.blocks.get(line.block) else { continue };
        let Some(text) = b.as_str().get(line.start..line.end) else { continue };
        let st = b.style_at(line.start);
        let Some(f) = builtin_face(st.family == Family::Mono, st.bold) else { continue };
        let x = sx as i32 + m;
        let y = sy as i32 + m + line.y as i32;
        let _ = draw_text_with(f, fb.pixels, stride, w, h, x, y, text, st.color, st.size_px);
    }
}
