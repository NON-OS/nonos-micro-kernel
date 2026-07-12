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

use super::layout::{CONTENT_X, FOOTER_H, PAD_X};
use super::state::{Mode, State};
use super::theme::{FOREGROUND, HEADER_BG, LINE, MUTED};

const HINT: &[u8] = b"n new   m dir   r rename   d del   c/x/p copy   / find   ? help";

pub fn paint_footer(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width;
    let cw = w.saturating_sub(CONTENT_X);
    let left = CONTENT_X + PAD_X;
    let y = fb.height.saturating_sub(FOOTER_H);
    fb.fill_rect(CONTENT_X, y, cw, FOOTER_H, HEADER_BG);
    fb.fill_rect(CONTENT_X, y, cw, 1, LINE);
    let ty = y + FOOTER_H.saturating_sub(8) / 2;
    let adv = fb.glyph_advance();

    // status (and live prompt/filter input) on the left
    fb.text(left, ty, state.status, MUTED);
    match state.mode {
        Mode::Prompt(_) | Mode::Filter => {
            let x = left + (state.status.len() as u32 + 2) * adv;
            let text = if matches!(state.mode, Mode::Filter) {
                state.filter.as_bytes()
            } else {
                state.input.as_bytes()
            };
            fb.text(x, ty, text, FOREGROUND);
        }
        _ => {
            // key hints on the right when there is room
            let hw = HINT.len() as u32 * adv;
            if w > hw + 360 {
                fb.text(w.saturating_sub(PAD_X + hw), ty, HINT, MUTED);
            }
        }
    }
}
