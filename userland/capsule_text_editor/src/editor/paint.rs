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

use super::clamp_scroll::clamp_scroll;
use super::end_position::end_position;
use super::layout::{wrap_cols, FIRST_LINE_Y, GLYPH_ADVANCE, LINE_HEIGHT, TEXT_LEFT};
use super::state::State;
use super::theme::{BACKGROUND, FOREGROUND, MUTED, TITLE};
use super::visible_rows::visible_rows;

pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    state.visible_rows = visible_rows(fb.height);
    state.wrap_cols = wrap_cols(fb.width);
    clamp_scroll(state, state.visible_rows);
    fb.clear(BACKGROUND);
    fb.text(TEXT_LEFT, 18, b"text_editor", TITLE);
    fb.text(TEXT_LEFT, 38, &state.path[..state.path_len], MUTED);
    if state.prompt.is_some() {
        fb.text(TEXT_LEFT + state.path_len as u32 * GLYPH_ADVANCE, 38, b"_", FOREGROUND);
    }
    fb.text(TEXT_LEFT, 58, state.status, MUTED);
    let mut col: u32 = 0;
    let mut line: u32 = 0;
    let text = core::str::from_utf8(&state.buf[..state.len]).unwrap_or("");
    for ch in text.chars() {
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
        let glyph = if ch.is_ascii() { [ch as u8, 0, 0, 0] } else { [b'?', 0, 0, 0] };
        let row = line - state.scroll_line;
        let ty = FIRST_LINE_Y + row * LINE_HEIGHT;
        fb.text(TEXT_LEFT + col * GLYPH_ADVANCE, ty, &glyph[..1], FOREGROUND);
        col += 1;
    }
    let (caret_line, caret_col) = end_position(&state.buf[..state.len], state.wrap_cols);
    if caret_line >= state.scroll_line && caret_line < state.scroll_line + state.visible_rows {
        let row = caret_line - state.scroll_line;
        let x = TEXT_LEFT + caret_col * GLYPH_ADVANCE;
        let y = FIRST_LINE_Y + row * LINE_HEIGHT;
        fb.text(x, y, b"_", FOREGROUND);
    }
}
