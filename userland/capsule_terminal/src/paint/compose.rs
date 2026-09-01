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
use super::constants::{BODY_PAD_TOP, HEADER_H, TEXT_LEFT};
use super::draw_grid::{draw_grid, draw_grid_cursor};
use super::draw_input_line::draw_input_line;
use super::fetch::draw_fetch;
use super::footer::{draw_footer, footer_h};
use super::header::draw_header;
use super::metrics::Metrics;
use super::rail_left;
use super::rail_right;
use crate::layout::{compute, Chrome, Layout, Rails};
use crate::palette::{Index, Palette};
use crate::rail::Rail;
use crate::term::prefs::types::Project;
use crate::term::state::State;
use crate::term::theme::types::Theme;

const RAIL_W: u32 = 256;
const RAIL_L: u32 = 240;

pub fn paint_tabs(
    tabs: &[State],
    active: usize,
    fb: &mut PaintBuffer,
    t: &Theme,
    font_scale: u32,
    rail: &Rail,
    projects: &[Project],
    monitor: bool,
    pal: &Palette,
) -> Layout {
    let l = paint(&tabs[active], fb, t, font_scale, rail, monitor);
    if l.left_rail.w > 0 {
        rail_left::draw(fb, l.left_rail, tabs, active, projects, t);
    }
    if pal.open {
        let ix = Index::build(tabs, active, projects);
        super::palette::draw(fb, l.body, pal, &ix, t);
    }
    l
}

pub fn paint(
    state: &State,
    fb: &mut PaintBuffer,
    t: &Theme,
    font_scale: u32,
    rail: &Rail,
    monitor: bool,
) -> Layout {
    fb.clear(t.bg);
    draw_header(state, fb, t);
    let m = Metrics::new(fb, font_scale);
    let chrome = Chrome {
        titlebar_h: HEADER_H,
        tabstrip_h: 0,
        body_pad_top: BODY_PAD_TOP,
        footer_h: footer_h(),
        text_left: TEXT_LEFT,
        row_h: m.lh,
    };
    let right = if monitor { RAIL_W } else { 0 };
    let l = compute(fb.width, fb.height, &chrome, Rails { left: RAIL_L, right });
    let text_x = l.body.x + chrome.text_left;
    let text_r = (l.body.x + l.body.w).saturating_sub(chrome.text_left);
    let alt = state.scrollback.grid.alternate;
    if alt {
        draw_grid(&state.scrollback.grid, fb, text_x, l.body.y, l.footer.y, text_r, m, t);
        draw_grid_cursor(&state.scrollback.grid, fb, text_x, l.body.y, m, t);
    } else if state.fresh {
        draw_fetch(state, fb, text_x, l.body.y, text_r, t);
    } else {
        draw_block_chrome(state, fb, text_x, l.body.y, l.input.y, text_r, &m, t);
        draw_grid(&state.scrollback.grid, fb, text_x, l.body.y, l.input.y, text_r, m, t);
    }
    if !alt {
        draw_input_line(state, fb, l.input, m, t);
    }
    if l.right_rail.w > 0 {
        rail_right::draw(fb, l.right_rail, rail, t);
    }
    draw_footer(fb, t);
    l
}
