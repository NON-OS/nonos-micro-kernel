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

use crate::ui::geometry::{Layout, Rect};
use crate::ui::sprite::{gradient_art, note};

use super::palette::{rgb24, ACCENT, GROOVE, MUTED, TEXT};

pub fn paint_header(fb: &mut PaintBuffer, l: &Layout, lib_active: bool) {
    let lg = &l.logo;
    let tile = gradient_art(64);
    fb.blit_rgba8_scaled(lg.x, lg.y, lg.w, lg.h, &tile.rgba, tile.w, tile.h);
    let gs = lg.w * 6 / 10;
    let nt = note(48, rgb24(TEXT));
    fb.blit_rgba8_scaled(lg.x + lg.w / 5, lg.y + lg.h / 5, gs, gs, &nt.rgba, nt.w, nt.h);

    let tx = (lg.x + lg.w + lg.w / 3) as i32;
    fb.text_ttf(tx, l.header.y as i32, "Resonare", TEXT, (lg.h as f32) * 0.62);
    fb.text(tx as u32, l.header.y + lg.h * 62 / 100, b"Audio player", MUTED);

    tab(fb, &l.now_tab, b"Now Playing", !lib_active);
    tab(fb, &l.lib_tab, b"Library", lib_active);
}

fn tab(fb: &mut PaintBuffer, r: &Rect, label: &[u8], active: bool) {
    let bg = if active { ACCENT } else { GROOVE };
    let fg = if active { TEXT } else { MUTED };
    fb.fill_rect(r.x, r.y, r.w, r.h, bg);
    let lx = r.x + r.w.saturating_sub(label.len() as u32 * 6) / 2;
    fb.text(lx, r.y + r.h / 2 - 4, label, fg);
}
