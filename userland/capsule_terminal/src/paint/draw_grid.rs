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

use super::constants::LINE_HEIGHT;
use crate::term::dimensions::{COLS, VISIBLE_ROWS};
use crate::term::grid::cell::F_REVERSE;
use crate::term::grid::types::Grid;
use crate::term::theme::{BACKGROUND, CURSOR};
use crate::term::vt::color::{ansi_to_argb, DEFAULT_BG};

pub fn draw_grid_cursor(g: &Grid, fb: &mut PaintBuffer, ox: u32, oy: u32) {
    if !g.cursor_visible {
        return;
    }
    let adv = fb.glyph_advance();
    let x = ox + g.x as u32 * adv;
    let y = oy + g.y as u32 * LINE_HEIGHT;
    fb.fill_rect(x, y, adv, LINE_HEIGHT, CURSOR);
    let ch = g.cells[Grid::idx(g.x, g.y)].ch;
    if ch != b' ' {
        fb.text(x, y, &[ch], BACKGROUND);
    }
}

pub fn draw_grid(g: &Grid, fb: &mut PaintBuffer, ox: u32, oy: u32, max_y: u32) {
    let adv = fb.glyph_advance();
    for row in 0..VISIBLE_ROWS {
        let y = oy + row as u32 * LINE_HEIGHT;
        if y + LINE_HEIGHT > max_y {
            break;
        }
        let rowcells = g.visible_row(row);
        for col in 0..COLS {
            let cell = rowcells[col];
            let x = ox + col as u32 * adv;
            let mut fg = ansi_to_argb(cell.fg);
            let mut bg = ansi_to_argb(cell.bg);
            let reverse = cell.flags & F_REVERSE != 0;
            if reverse {
                core::mem::swap(&mut fg, &mut bg);
            }
            if cell.bg != DEFAULT_BG || reverse {
                fb.fill_rect(x, y, adv, LINE_HEIGHT, bg);
            }
            if cell.ch != b' ' {
                fb.text(x, y, &[cell.ch], fg);
            }
        }
    }
}
