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

use super::palette_row::draw_row;
use super::rail_text::{clipped, left, lh, RAIL_GAP, RAIL_PAD, RAIL_PX};
use super::tokens::{PALETTE_PANEL, SCRIM};
use crate::layout::Rect;
use crate::palette::{filter, panel, query_row, row, rows_fit, Index, Palette, MAX_ROWS};
use crate::term::theme::types::Theme;

const RADIUS: u32 = 10;
const SHADOW: u32 = 8;

/// The first overlay layer in this window. Every pixel is a blend: the panel
/// sits on paint the frame already laid down, so a raw write would replace it
/// and punch a hole through to the wallpaper.
pub fn draw(fb: &mut PaintBuffer, body: Rect, pal: &Palette, ix: &Index, t: &Theme) {
    fb.blend_rect(body.x, body.y, body.w, body.h, SCRIM);
    let row_h = lh() + RAIL_GAP;
    let mut hits = [0usize; MAX_ROWS];
    let n = filter(ix.slice(), pal.needle(), &mut hits);
    let p = panel(body, row_h, n as u32);
    fb.shadow_round(p.x, p.y, p.w, p.h, RADIUS, SHADOW, SCRIM);
    fb.fill_round(p.x, p.y, p.w, p.h, RADIUS, PALETTE_PANEL);
    let q = query_row(p, row_h);
    draw_query(fb, q, pal, t);
    fb.blend_rect(q.x, q.y + row_h, q.w, 1, t.chrome_edge);
    let visible = (n as u32).min(rows_fit(p, row_h));
    for i in 0..visible {
        let e = &ix.slice()[hits[i as usize]];
        draw_row(fb, row(p, i, row_h), e, i as usize == pal.sel.min(n.saturating_sub(1)), t);
    }
}

fn draw_query(fb: &mut PaintBuffer, q: Rect, pal: &Palette, t: &Theme) {
    let y = (q.y + (q.h.saturating_sub(lh())) / 2) as i32;
    let w = left(fb, q.x + RAIL_PAD, y, ">", t.accent);
    let text = core::str::from_utf8(pal.needle()).unwrap_or("");
    let x = q.x + RAIL_PAD + w + RAIL_GAP;
    if text.is_empty() {
        clipped(fb, x, y, q.w, "Run a command, open a session or a project", t.dim);
        return;
    }
    clipped(fb, x, y, q.x + q.w - x, text, t.fg);
    let cw = super::fit_text::width_of(fb, text, RAIL_PX);
    fb.blend_rect(x + cw + 2, y as u32, 2, lh(), t.accent);
}
