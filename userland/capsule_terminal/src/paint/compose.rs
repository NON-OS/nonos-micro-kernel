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

use super::constants::{BODY_TOP, FOOTER_H, LINE_HEIGHT, TEXT_LEFT};
use super::draw_grid::{draw_grid, draw_grid_cursor};
use super::draw_input_line::draw_input_line;
use super::fetch::draw_fetch;
use super::footer::draw_footer;
use super::header::draw_header;
use crate::term::state::State;
use crate::term::theme::BACKGROUND;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    draw_header(state, fb);
    let alt = state.scrollback.grid.alternate;
    let input_y = fb.height.saturating_sub(FOOTER_H + LINE_HEIGHT);
    if alt {
        let body_max = fb.height.saturating_sub(FOOTER_H);
        draw_grid(&state.scrollback.grid, fb, TEXT_LEFT, BODY_TOP, body_max);
        draw_grid_cursor(&state.scrollback.grid, fb, TEXT_LEFT, BODY_TOP);
    } else if state.fresh {
        draw_fetch(state, fb);
    } else {
        draw_grid(&state.scrollback.grid, fb, TEXT_LEFT, BODY_TOP, input_y);
    }
    if !alt {
        draw_input_line(state, fb, input_y);
    }
    draw_footer(fb);
}
