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

//! Paint the code pane: the line-number gutter, the current-line band, crisp
//! monospace body text with syntax colours over the selection, and the caret.
//! Everything is drawn relative to the pane rectangle the shell assigns, so the
//! editor sits to the right of the sidebar and below the tab strip.

use nonos_app_skeleton::PaintBuffer;

use super::clamp_scroll::clamp_scroll;
use super::highlight;
use super::layout::{text_left, wrap_cols, BODY_PX, GUTTER_PX, GUTTER_W, LINE_HEIGHT, PAD_TOP};
use super::position_at::position_at;
use super::state::State;
use super::theme::{BACKGROUND, CARET, CURRENT_LINE, GUTTER_BG, GUTTER_CUR, GUTTER_FG, SELECTION};

pub(super) fn paint_document(state: &mut State, fb: &mut PaintBuffer) {
    let adv = fb.measure_ttf_mono("M", BODY_PX).max(1) as u32;
    state.glyph_advance = adv;
    state.visible_rows = (state.pane_h / LINE_HEIGHT).max(1);
    state.wrap_cols = wrap_cols(state.pane_w, adv);
    clamp_scroll(state, state.visible_rows);
    state.caret = state.caret.min(state.len);

    let (caret_line, caret_col) =
        position_at(&state.buf[..state.len], state.wrap_cols, state.caret);

    fb.fill_rect(state.pane_x, state.pane_y, state.pane_w, state.pane_h, BACKGROUND);
    paint_current_line(state, fb, caret_line);
    paint_gutter(state, fb, caret_line);
    paint_body(state, fb, adv);
    paint_caret(state, fb, adv, caret_line, caret_col);
}

// The caret's line and column, for the status bar; assumes wrap_cols is set,
// which paint_document does before the shell asks for this.
pub(super) fn caret_position(state: &State) -> (u32, u32) {
    position_at(&state.buf[..state.len], state.wrap_cols, state.caret)
}

fn in_view(state: &State, line: u32) -> bool {
    line >= state.scroll_line && line < state.scroll_line + state.visible_rows
}

fn row_top(state: &State, line: u32) -> u32 {
    state.pane_y + PAD_TOP + (line - state.scroll_line) * LINE_HEIGHT
}

fn paint_current_line(state: &State, fb: &mut PaintBuffer, caret_line: u32) {
    if state.sel_range().is_none() && in_view(state, caret_line) {
        let x = state.pane_x + GUTTER_W;
        fb.fill_rect(
            x,
            row_top(state, caret_line),
            state.pane_w - GUTTER_W,
            LINE_HEIGHT,
            CURRENT_LINE,
        );
    }
}

fn paint_gutter(state: &State, fb: &mut PaintBuffer, caret_line: u32) {
    fb.fill_rect(state.pane_x, state.pane_y, GUTTER_W, state.pane_h, GUTTER_BG);
    for row in 0..state.visible_rows {
        let ln = state.scroll_line + row + 1;
        let mut buf = [0u8; 10];
        let n = fmt_u32(ln, &mut buf);
        let s = core::str::from_utf8(&buf[..n]).unwrap_or("");
        let nw = fb.measure_ttf(s, GUTTER_PX).max(0) as u32;
        let x = state.pane_x + GUTTER_W.saturating_sub(nw + 12);
        let color = if ln == caret_line + 1 { GUTTER_CUR } else { GUTTER_FG };
        let y = (state.pane_y + PAD_TOP + row * LINE_HEIGHT + 4) as i32;
        let _ = fb.text_ttf(x as i32, y, s, color, GUTTER_PX);
    }
}

fn paint_body(state: &State, fb: &mut PaintBuffer, adv: u32) {
    let toks = highlight::classify(&state.buf[..state.len]);
    let sel = state.sel_range();
    let text = core::str::from_utf8(&state.buf[..state.len]).unwrap_or("");
    let left = text_left(state.pane_x);
    let mut col = 0u32;
    let mut line = 0u32;
    let mut chbuf = [0u8; 4];
    for (bi, ch) in text.char_indices() {
        if ch == '\n' || col == state.wrap_cols {
            line += 1;
            col = 0;
            if ch == '\n' {
                continue;
            }
        }
        if line < state.scroll_line {
            col += 1;
            continue;
        }
        if line >= state.scroll_line + state.visible_rows {
            break;
        }
        let ty = row_top(state, line);
        let x = left + col * adv;
        if let Some((s, e)) = sel {
            if bi >= s && bi < e {
                fb.fill_rect(x, ty, adv, LINE_HEIGHT, SELECTION);
            }
        }
        if ch > ' ' && ch != '\u{7f}' {
            let color = highlight::color(toks.get(bi).copied().unwrap_or(highlight::Tok::Text));
            let s = ch.encode_utf8(&mut chbuf);
            let _ = fb.text_ttf_mono(x as i32, ty as i32, s, color, BODY_PX);
        }
        col += 1;
    }
}

fn paint_caret(state: &State, fb: &mut PaintBuffer, adv: u32, caret_line: u32, caret_col: u32) {
    if in_view(state, caret_line) {
        let x = text_left(state.pane_x) + caret_col * adv;
        fb.fill_rect(x, row_top(state, caret_line), 2, LINE_HEIGHT, CARET);
    }
}

fn fmt_u32(mut v: u32, out: &mut [u8; 10]) -> usize {
    if v == 0 {
        out[0] = b'0';
        return 1;
    }
    let mut tmp = [0u8; 10];
    let mut n = 0;
    while v > 0 && n < 10 {
        tmp[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    for i in 0..n {
        out[i] = tmp[n - 1 - i];
    }
    n
}
