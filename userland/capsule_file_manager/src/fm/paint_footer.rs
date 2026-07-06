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

use super::paint_metrics::{DATE_END, GLYPH_W, STATUS_Y, TEXT_LEFT};
use super::paint_right::paint_right;
use super::state::{Mode, State};
use super::theme::{FOREGROUND, MUTED};

pub fn paint_footer(state: &State, fb: &mut PaintBuffer) {
    let count = alloc::format!("{}/{}", state.cursor + 1, state.entries.len().max(1));
    paint_right(fb, DATE_END, 38, count.as_bytes(), MUTED);
    fb.text(TEXT_LEFT, STATUS_Y, state.status, MUTED);
    match state.mode {
        Mode::Prompt(_) | Mode::Filter => {
            let x = TEXT_LEFT + GLYPH_W * state.status.len() as u32;
            let text = if matches!(state.mode, Mode::Filter) {
                state.filter.as_bytes()
            } else {
                state.input.as_bytes()
            };
            fb.text(x, STATUS_Y, text, FOREGROUND);
        }
        _ => {}
    }
}
