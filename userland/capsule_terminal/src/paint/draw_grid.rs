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

//! Draw the character grid with crisp monospace TrueType glyphs. Each cell is
//! laid out on the measured advance and painted with `text_ttf_mono`, so the
//! body reads like a real terminal rather than a scaled bitmap.

use nonos_app_skeleton::PaintBuffer;

use super::metrics::Metrics;
use crate::term::dimensions::{COLS, VISIBLE_ROWS};
use crate::term::grid::cell::{F_REVERSE, F_WIDE_TAIL};
use crate::term::grid::types::Grid;
use crate::term::theme::{BACKGROUND, CURSOR};
use crate::term::vt::color::DEFAULT_BG;

fn glyph(fb: &mut PaintBuffer, x: u32, y: u32, ch: char, argb: u32, px: f32) {
    // Blanks and control characters have no glyph. Everything else is handed
    // to the face, which covers far more than ASCII and draws .notdef for
    // what it does not have, so an unmapped character is visibly missing
    // rather than silently absent.
    if ch == ' ' || (ch as u32) < 0x20 || ch as u32 == 0x7f {
        return;
    }
    let mut buf = [0u8; 4];
    let s = ch.encode_utf8(&mut buf);
    let _ = fb.text_ttf_mono(x as i32, y as i32, s, argb, px);
}

pub fn draw_grid_cursor(g: &Grid, fb: &mut PaintBuffer, ox: u32, oy: u32, m: Metrics) {
    if !g.cursor_visible {
        return;
    }
    let x = ox + g.x as u32 * m.adv;
    let y = oy + g.y as u32 * m.lh;
    fb.fill_rect(x, y, m.adv, m.lh, CURSOR);
    glyph(fb, x, y, g.cells[Grid::idx(g.x, g.y)].ch, BACKGROUND, m.px);
}

pub fn draw_grid(g: &Grid, fb: &mut PaintBuffer, ox: u32, oy: u32, max_y: u32, m: Metrics) {
    for row in 0..VISIBLE_ROWS {
        let y = oy + row as u32 * m.lh;
        if y + m.lh > max_y {
            break;
        }
        let rowcells = g.visible_row(row);
        for (col, cell) in rowcells.iter().enumerate().take(COLS) {
            let x = ox + col as u32 * m.adv;
            if x + m.adv > fb.width {
                break;
            }
            let has_bg = cell.bg != DEFAULT_BG;
            let mut fg = cell.fg;
            // A default (transparent) background resolves to the terminal
            // backdrop when it needs to become a visible colour, so reverse
            // video and explicit fills both read correctly.
            let mut bg = if has_bg { cell.bg } else { BACKGROUND };
            let reverse = cell.flags & F_REVERSE != 0;
            if reverse {
                core::mem::swap(&mut fg, &mut bg);
            }
            if has_bg || reverse {
                fb.fill_rect(x, y, m.adv, m.lh, bg);
            }
            // The right half of a wide character carries the background and
            // nothing else. Its glyph was drawn by the cell before it, which
            // had both columns to draw into.
            if cell.flags & F_WIDE_TAIL == 0 {
                glyph(fb, x, y, cell.ch, fg, m.px);
            }
        }
    }
}
