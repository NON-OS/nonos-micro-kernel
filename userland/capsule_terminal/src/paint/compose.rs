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

use super::block_chrome::draw_block_chrome;
use super::constants::{BODY_TOP, FOOTER_H, TEXT_LEFT};
use super::draw_grid::{draw_grid, draw_grid_cursor};
use super::draw_input_line::draw_input_line;
use super::fetch::draw_fetch;
use super::footer::draw_footer;
use super::header::draw_header;
use super::metrics::Metrics;
use crate::term::state::State;

pub fn paint_tabs(tabs: &[State], active: usize, fb: &mut PaintBuffer) {
    paint(&tabs[active], fb);
    crate::paint::draw_tabstrip(tabs, active, fb);
}

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(state.bg);
    draw_header(state, fb);
    let m = Metrics::new(fb, state.font_scale);
    let alt = state.scrollback.grid.alternate;
    let input_y = fb.height.saturating_sub(FOOTER_H + m.lh);
    if alt {
        let body_max = fb.height.saturating_sub(FOOTER_H);
        draw_grid(&state.scrollback.grid, fb, TEXT_LEFT, BODY_TOP, body_max, m);
        draw_grid_cursor(&state.scrollback.grid, fb, TEXT_LEFT, BODY_TOP, m);
    } else if state.fresh {
        draw_fetch(state, fb);
    } else {
        draw_block_chrome(state, fb, TEXT_LEFT, BODY_TOP, input_y, &m);
        draw_grid(&state.scrollback.grid, fb, TEXT_LEFT, BODY_TOP, input_y, m);
    }
    if !alt {
        draw_input_line(state, fb, input_y, m);
    }
    draw_footer(fb, state.bg);
}
