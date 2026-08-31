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

use crate::pm::state::State;
use crate::pm::theme::{ACCENT, DANGER, MUTED, RULE_SOFT};

use super::super::chrome::Rect;
use super::super::metrics::{BODY_PX, NUM_PX, PANEL_HEAD_H, PANEL_PAD};
use super::super::spark;
use super::super::text;
use super::cpu::panel;
use super::ovw_cards::sub_n;

// Gridlines the eye reads a percentage off, and the saturation threshold the
// mock marks in red. All are opaque hairlines, so fill_rect is correct here.
const GRID: [u32; 3] = [25, 50, 75];
const SATURATION: u32 = 95;
const DASH: u32 = 6;

pub(super) fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect, h: u32) {
    let rect = Rect { x: r.x, y: r.y, w: r.w, h };
    let top = panel(fb, &rect, b"TOTAL LOAD");
    let (x, w) = (r.x + PANEL_PAD, r.w.saturating_sub(PANEL_PAD * 2));
    let py = top + PANEL_PAD;
    let ph = (r.y + h).saturating_sub(py + PANEL_PAD);
    for pct in GRID {
        fb.fill_rect(x, py + ph - ph * pct / 100, w, 1, RULE_SOFT);
    }
    spark::cpu(fb, x, py, w, ph, &state.history.total, ACCENT);
    let sat = py + ph - ph * SATURATION / 100;
    dashed(fb, x, sat, w, DANGER);
    text::left(fb, x, sat + DASH, b"SATURATION", DANGER, BODY_PX);
    legend(state, fb, &rect);
}

// Dashed rather than solid, because saturation is a threshold the load may
// cross, not a value the chart plots.
fn dashed(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, argb: u32) {
    let mut px = x;
    while px + DASH < x + w {
        fb.fill_rect(px, y, DASH, 1, argb);
        px += DASH * 2;
    }
}

// Peak and mean are placed from the panel's right edge by measurement, so a
// three-digit peak never shoves the caption that sits on the same line.
fn legend(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let ring = &state.history.total;
    let n = ring.len().max(1) as u32;
    let mean = (0..ring.len()).map(|i| ring.cpu_at(i) as u32).sum::<u32>() / n;
    let top = text::centred_top(r.y, PANEL_HEAD_H, NUM_PX);
    let right = r.x + r.w.saturating_sub(PANEL_PAD);
    let mut buf = [0u8; 24];
    let len = sub_n(&mut buf, b"mean ", mean, b"%");
    text::mono_right(fb, right, top, &buf[..len], MUTED, NUM_PX);
    let used = text::mono_width(fb, &buf[..len], NUM_PX) + PANEL_PAD;
    let len = sub_n(&mut buf, b"peak ", ring.peak_cpu() as u32, b"%");
    text::mono_right(fb, right.saturating_sub(used), top, &buf[..len], MUTED, NUM_PX);
}
