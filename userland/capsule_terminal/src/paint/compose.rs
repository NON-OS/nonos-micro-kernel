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
use super::constants::{BODY_TOP, FOOTER_H, HEADER_H, TEXT_LEFT};
use super::draw_grid::{draw_grid, draw_grid_cursor};
use super::draw_input_line::draw_input_line;
use super::fetch::draw_fetch;
use super::footer::draw_footer;
use super::header::draw_header;
use super::metrics::Metrics;
use super::tabstrip::STRIP_H;
use crate::layout::{compute, Chrome, Rails};
use crate::term::state::State;

pub fn paint_tabs(tabs: &[State], active: usize, fb: &mut PaintBuffer) {
    paint(&tabs[active], fb);
    crate::paint::draw_tabstrip(tabs, active, fb);
}

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(state.bg);
    draw_header(state, fb);
    let m = Metrics::new(fb, state.font_scale);
    let chrome = Chrome {
        titlebar_h: HEADER_H,
        tabstrip_h: STRIP_H,
        body_pad_top: BODY_TOP - HEADER_H - STRIP_H,
        footer_h: FOOTER_H,
        text_left: TEXT_LEFT,
        row_h: m.lh,
    };
    let l = compute(fb.width, fb.height, &chrome, Rails { left: 0, right: 0 });
    let text_x = l.body.x + chrome.text_left;
    let alt = state.scrollback.grid.alternate;
    if alt {
        draw_grid(&state.scrollback.grid, fb, text_x, l.body.y, l.footer.y, m);
        draw_grid_cursor(&state.scrollback.grid, fb, text_x, l.body.y, m);
    } else if state.fresh {
        draw_fetch(state, fb);
    } else {
        draw_block_chrome(state, fb, text_x, l.body.y, l.input.y, &m);
        draw_grid(&state.scrollback.grid, fb, text_x, l.body.y, l.input.y, m);
    }
    if !alt {
        draw_input_line(state, fb, l.input.y, m);
    }
    draw_footer(fb, state.bg);
}
