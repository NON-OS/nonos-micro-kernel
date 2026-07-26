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

use crate::model::PlayerView;
use crate::ui::format::{state_label, time_readout};
use crate::ui::geometry::Layout;
use crate::ui::sprite::gradient_art;
use crate::waveform::Waveform;

use super::bars::paint_waveform;
use super::meter::paint_vu;
use super::palette::{ACCENT, GROOVE, MUTED, PANEL, TEXT};

pub fn paint_panel(fb: &mut PaintBuffer, v: &PlayerView, wf: &Waveform, l: &Layout) {
    fb.fill_rect(l.panel.x, l.panel.y, l.panel.w, l.panel.h, PANEL);
    let art = gradient_art(128);
    fb.blit_rgba8_scaled(l.art.x, l.art.y, l.art.w, l.art.h, &art.rgba, art.w, art.h);
    paint_badge(fb, v, l);
    let px = (l.title.h as f32) * 0.92;
    fb.text_ttf(l.title.x as i32, l.title.y as i32, v.title.as_str(), TEXT, px);
    fb.text(l.artist.x, l.artist.y, v.artist.as_bytes(), MUTED);
    paint_waveform(fb, v, wf, &l.waveform);
    let mut buf = [0u8; 16];
    let n = time_readout(&mut buf, v.pos_ms, v.dur_ms);
    fb.text(l.waveform.x, l.waveform.y + l.waveform.h + 6, &buf[..n], MUTED);
    paint_vu(fb, v, &l.vu);
}

fn paint_badge(fb: &mut PaintBuffer, v: &PlayerView, l: &Layout) {
    let bw = (v.format.len() as u32 * 8 + 16).min(l.badge.w.max(40));
    fb.fill_rect(l.badge.x, l.badge.y, bw, l.badge.h, GROOVE);
    let ty = l.badge.y + l.badge.h / 2 - 4;
    fb.text(l.badge.x + 8, ty, v.format.as_bytes(), ACCENT);
    fb.text(l.badge.x + bw + 10, ty, state_label(v.state), MUTED);
}
