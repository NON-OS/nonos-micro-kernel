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

use crate::snake::theme::TRACK_BG;

use super::metrics::GAP_TIGHT;
use super::rect::Rect;

pub const BAR_RADIUS: u32 = 3;

// The caller passes a numerator and a denominator rather than a fraction: this
// capsule keeps floats out of the paint path, and the product is taken in u64
// because an elapsed figure in milliseconds outgrows u32 in a long session. A
// fill narrower than the bar is squared off, since a radius wider than the fill
// renders as a smear.
pub fn meter(fb: &mut PaintBuffer, r: Rect, num: u64, den: u64, tint: u32) {
    let radius = BAR_RADIUS.min(r.3 / 2);
    fb.fill_round(r.0, r.1, r.2, r.3, radius, TRACK_BG);
    if den == 0 || r.2 == 0 {
        return;
    }
    let fill = (num.min(den) * r.2 as u64 / den) as u32;
    if fill >= r.3 {
        fb.fill_round(r.0, r.1, fill, r.3, radius, tint);
    } else if fill > 0 {
        fb.blend_rect(r.0, r.1, fill, r.3, tint);
    }
}

// Lives and level progress read as discrete marks, not a percentage, so they
// get pips: `count` slots across the rect with the first `filled` lit.
pub fn pips(fb: &mut PaintBuffer, r: Rect, count: usize, filled: usize, tint: u32) {
    if count == 0 {
        return;
    }
    let n = count as u32;
    let w = r.2.saturating_sub(GAP_TIGHT * (n - 1)) / n;
    let radius = BAR_RADIUS.min(r.3 / 2);
    for index in 0..count {
        let x = r.0 + index as u32 * (w + GAP_TIGHT);
        let argb = if index < filled { tint } else { TRACK_BG };
        fb.fill_round(x, r.1, w, r.3, radius, argb);
    }
}
