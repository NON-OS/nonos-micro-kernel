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

//! The line the reader has not typed yet.

use nonos_app_skeleton::PaintBuffer;

use super::line_text::text;
use crate::term::state::State;
use crate::term::theme::DIM;

/// Draw the rest of the most recent line that starts with what is typed.
///
/// It sits after the cursor in the quiet colour, so it reads as an offer
/// rather than as text that is already there. A reader who keeps typing
/// simply covers it; one who wants it presses the accept key.
///
/// Nothing is drawn while the cursor is away from the end of the line. A
/// suggestion appended to a line being edited in the middle would describe a
/// command nobody could produce by accepting it.
pub fn draw_suggestion(
    state: &State,
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    adv: u32,
    px: f32,
    room_cells: usize,
) {
    let body = state.line.as_bytes();
    if state.line.cursor != body.len() {
        return;
    }
    let Some(full) = state.history.suggest(body) else {
        return;
    };
    let tail = &full[body.len()..];
    // A tail that runs past the edge is cut to what fits rather than wrapping
    // into the row below, which belongs to the scrollback.
    let shown = tail.len().min(room_cells);
    if shown == 0 {
        return;
    }
    text(fb, x, y, &tail[..shown], DIM, adv, px);
}
