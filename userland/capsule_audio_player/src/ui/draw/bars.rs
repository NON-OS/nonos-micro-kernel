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
use crate::ui::geometry::Rect;
use crate::waveform::Waveform;

use super::palette::{ACCENT, GROOVE, MUTED};

pub fn paint_bar(fb: &mut PaintBuffer, r: &Rect, num: u32, den: u32) {
    let cy = r.y + r.h / 2;
    fb.fill_rect(r.x, cy.saturating_sub(1), r.w, 3, GROOVE);
    let fill = if den == 0 { 0 } else { (r.w as u64 * num as u64 / den as u64) as u32 };
    fb.fill_rect(r.x, cy.saturating_sub(1), fill, 3, ACCENT);
    let knob = r.x + fill.min(r.w);
    fb.fill_rect(knob.saturating_sub(2), cy.saturating_sub(4), 4, 9, ACCENT);
}

pub fn paint_waveform(fb: &mut PaintBuffer, v: &PlayerView, wf: &Waveform, r: &Rect) {
    let n = wf.buckets.len() as u32;
    if n == 0 {
        return;
    }
    let played = if v.dur_ms == 0 { 0 } else { (r.w as u64 * v.pos_ms as u64 / v.dur_ms as u64) as u32 };
    let cy = r.y + r.h / 2;
    let bw = (r.w / n).max(1).saturating_sub(1).max(1);
    for (i, &b) in wf.buckets.iter().enumerate() {
        let bx = r.x + (r.w * i as u32) / n;
        let bh = ((r.h * b as u32) / 255).max(2);
        let top = cy.saturating_sub(bh / 2);
        let color = if bx.saturating_sub(r.x) < played { ACCENT } else { MUTED };
        fb.fill_rect(bx, top, bw, bh, color);
    }
    fb.fill_rect((r.x + played).saturating_sub(1), r.y, 2, r.h, ACCENT);
}
