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

use crate::model::PlayerView;
use crate::ui::format::{state_label, time_readout};
use crate::ui::geometry::{Layout, Rect};
use crate::ui::sprite::gradient_art;
use crate::waveform::Waveform;

use super::bars::paint_waveform;
use super::chip::chip;
use super::meter::paint_vu;
use super::palette::{ACCENT_DIM, ACCENT_LIGHT, DIM, LINE, MUTED, PANEL, TEXT};

pub fn paint_panel(fb: &mut PaintBuffer, v: &PlayerView, wf: &Waveform, l: &Layout) {
    chip(fb, &l.panel, l.panel.h * 8 / 100, PANEL, LINE);
    let art = gradient_art(128);
    fb.blit_rgba8_scaled(l.art.x, l.art.y, l.art.w, l.art.h, &art.rgba, art.w, art.h);
    paint_badge(fb, v, l);
    let px = (l.title.h as f32) * 0.92;
    fb.text_ttf(l.title.x as i32, l.title.y as i32, v.title.as_str(), TEXT, px);
    fb.text_ttf(l.artist.x as i32, l.artist.y as i32, v.artist.as_str(), MUTED, (l.artist.h as f32) * 0.45);
    paint_waveform(fb, v, wf, &l.waveform);
    let mut buf = [0u8; 16];
    let n = time_readout(&mut buf, v.pos_ms, v.dur_ms);
    fb.text(l.waveform.x, l.waveform.y + l.waveform.h + 6, &buf[..n], DIM);
    paint_vu(fb, v, &l.vu);
}

fn paint_badge(fb: &mut PaintBuffer, v: &PlayerView, l: &Layout) {
    let bw = (v.format.len() as u32 * font_advance() + 16).min(l.badge.w.max(40));
    let (bx, by, bh) = (l.badge.x, l.badge.y, l.badge.h);
    let pill = Rect { x: bx, y: by, w: bw, h: bh };
    chip(fb, &pill, bh * 30 / 100, PANEL, ACCENT_DIM);
    let ty = by + bh / 2 - 4;
    fb.text(bx + 8, ty, v.format.as_bytes(), ACCENT_LIGHT);
    fb.text(bx + bw + 10, ty, state_label(v.state), MUTED);
}
