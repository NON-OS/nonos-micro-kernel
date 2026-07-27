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

use nonos_app_skeleton::{font_advance, PaintBuffer};

use crate::ui::geometry::{Layout, Rect};
use crate::ui::sprite::{gradient_logo, note};

use super::chip::chip;
use super::palette::{rgb24, ACCENT_DIM, ACCENT_LIGHT, ACCENT_WASH, BG, DIM, LINE_2, MUTED, TEXT};

pub fn paint_header(fb: &mut PaintBuffer, l: &Layout, lib_active: bool) {
    let lg = &l.logo;
    let tile = gradient_logo(64);
    fb.blit_rgba8_scaled(lg.x, lg.y, lg.w, lg.h, &tile.rgba, tile.w, tile.h);
    let gs = lg.w * 6 / 10;
    let nt = note(48, rgb24(TEXT));
    fb.blit_rgba8_scaled(lg.x + lg.w / 5, lg.y + lg.h / 5, gs, gs, &nt.rgba, nt.w, nt.h);

    let tx = (lg.x + lg.w + lg.w / 3) as i32;
    let px = (lg.h as f32) * 0.62;
    let pen = fb.text_ttf(tx, l.header.y as i32, "Reson", TEXT, px);
    fb.text_ttf(pen, l.header.y as i32, "are", ACCENT_LIGHT, px);
    fb.text(tx as u32, l.header.y + lg.h * 62 / 100, b"Audio player", DIM);

    tab(fb, &l.now_tab, b"Now Playing", !lib_active);
    tab(fb, &l.lib_tab, b"Library", lib_active);
}

fn tab(fb: &mut PaintBuffer, r: &Rect, label: &[u8], active: bool) {
    let (fill, border, fg) = if active {
        (ACCENT_WASH, ACCENT_DIM, ACCENT_LIGHT)
    } else {
        (BG, LINE_2, MUTED)
    };
    chip(fb, r, r.h / 2, fill, border);
    let lw = label.len() as u32 * font_advance();
    let lx = r.x + r.w.saturating_sub(lw) / 2;
    fb.text(lx, r.y + r.h / 2 - 4, label, fg);
}
