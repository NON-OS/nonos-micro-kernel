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

use nonos_app_skeleton::{measure_ttf, PaintBuffer};

use super::layout::{CONTENT_X, FOOTER_H, PAD_X};
use super::sel_summary::count_label;
use super::state::{Mode, State};
use super::theme::{CY, INK, INK3, LINE, PANEL};

const HINT: &str = "n new    m dir    r rename    d del    c/x/p copy    / find    ? help";
const FOOT_PX: f32 = 14.0;
const GAP: u32 = 10;

/// The status line and the hint strip share one band, and both are measured off
/// the same facade the rest of the chrome draws with, so a long status pushes
/// the caret exactly as far as the glyphs actually ran.
pub fn paint_footer(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width;
    let band = w.saturating_sub(CONTENT_X);
    let y = fb.height.saturating_sub(FOOTER_H);
    fb.fill_rect(CONTENT_X, y, band, FOOTER_H, PANEL);
    fb.fill_rect(CONTENT_X, y, band, 1, LINE);
    let mut left = CONTENT_X + PAD_X;
    let ty = (y + 8) as i32;
    if let Some(count) = count_label(state) {
        let _ = fb.text_ttf(left as i32, ty, count.as_str(), CY, FOOT_PX);
        left += measure_ttf(count.as_str(), FOOT_PX).max(0) as u32 + GAP;
    }
    let status = core::str::from_utf8(state.status).unwrap_or("");
    let _ = fb.text_ttf(left as i32, ty, status, INK3, FOOT_PX);
    match state.mode {
        Mode::Prompt(_) | Mode::Filter => {
            let typed = if matches!(state.mode, Mode::Filter) {
                state.filter.as_str()
            } else {
                state.input.as_str()
            };
            let x = left + measure_ttf(status, FOOT_PX).max(0) as u32 + GAP;
            let _ = fb.text_ttf(x as i32, ty, typed, INK, FOOT_PX);
        }
        _ => {
            let hw = measure_ttf(HINT, FOOT_PX).max(0) as u32;
            if w > CONTENT_X + PAD_X * 2 + hw {
                let _ = fb.text_ttf(w.saturating_sub(PAD_X + hw) as i32, ty, HINT, INK3, FOOT_PX);
            }
        }
    }
}
