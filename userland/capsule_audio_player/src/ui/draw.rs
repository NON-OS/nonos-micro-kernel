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
use crate::transport::State;
use crate::waveform::Waveform;

use super::geometry::{Layout, Rect};

pub const BG: u32 = 0xFF0E0E16;
pub const ACCENT: u32 = 0xFF7C5CFF;
pub const TEXT: u32 = 0xFFE8EEF4;
pub const MUTED: u32 = 0xFF6A6A80;
pub const GROOVE: u32 = 0xFF232334;

pub fn paint_bar(fb: &mut PaintBuffer, r: &Rect, num: u32, den: u32) {
    fb.fill_rect(r.x, r.y, r.w, r.h, GROOVE);
    let fill = if den == 0 { 0 } else { (r.w as u64 * num as u64 / den as u64) as u32 };
    fb.fill_rect(r.x, r.y, fill, r.h, ACCENT);
}

pub fn paint_waveform(fb: &mut PaintBuffer, v: &PlayerView, wf: &Waveform, r: &Rect) {
    let n = wf.buckets.len() as u32;
    if n == 0 {
        return;
    }
    let played = if v.dur_ms == 0 { 0 } else { (n as u64 * v.pos_ms as u64 / v.dur_ms as u64) as u32 };
    let cy = r.y + r.h / 2;
    let step = (r.w / n).max(1);
    for (i, &b) in wf.buckets.iter().enumerate() {
        let bx = r.x + (r.w * i as u32) / n;
        let bh = ((r.h * b as u32) / 255).max(1);
        let top = cy.saturating_sub(bh / 2);
        let color = if (i as u32) < played { ACCENT } else { MUTED };
        fb.fill_rect(bx, top, step.saturating_sub(1).max(1), bh, color);
    }
}

fn button(fb: &mut PaintBuffer, r: &Rect, label: &[u8]) {
    fb.fill_rect(r.x, r.y, r.w, r.h, GROOVE);
    fb.fill_rect(r.x, r.y, r.w, 1, ACCENT);
    fb.fill_rect(r.x, r.y + r.h - 1, r.w, 1, ACCENT);
    fb.fill_rect(r.x, r.y, 1, r.h, ACCENT);
    fb.fill_rect(r.x + r.w - 1, r.y, 1, r.h, ACCENT);
    let tx = r.x + r.w / 2 - (label.len() as u32 * 4).min(r.w / 2);
    let ty = r.y + r.h / 2 - 4;
    fb.text(tx, ty, label, TEXT);
}

pub fn paint_buttons(fb: &mut PaintBuffer, v: &PlayerView, l: &Layout) {
    button(fb, &l.back, b"<<");
    let play_label: &[u8] = if v.state == State::Playing { b"PAUS" } else { b"PLAY" };
    button(fb, &l.play, play_label);
    button(fb, &l.stop, b"STOP");
    button(fb, &l.fwd, b">>");
}
