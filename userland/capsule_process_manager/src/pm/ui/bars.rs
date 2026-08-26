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

use crate::pm::theme::{FOREGROUND, TRACK_BG};

use super::metrics::{BAR_H, BAR_ROW_H, BODY_PX, NUM_PX};
use super::text;

pub const BAR_RADIUS: u32 = 3;
pub const BAR_GAP: u32 = 12;

// The one horizontal meter every screen draws, so CPU, Memory and Authority
// cannot drift apart. The caller passes a numerator and a denominator instead of
// a fraction because this capsule keeps floats out of its paint path, and the
// product is taken in u64 since a resident figure in kb reaches ~4e9. Both the
// track and the fill blend, so an alpha-carrying tint cannot punch through the
// panel underneath the way a raw fill_rect would.
pub fn hbar(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, num: u64, den: u64, tint: u32) {
    let radius = BAR_RADIUS.min(h / 2);
    fb.fill_round(x, y, w, h, radius, TRACK_BG);
    if den == 0 || w == 0 {
        return;
    }
    let fill = (num.min(den) * w as u64 / den) as u32;
    if fill >= h {
        fb.fill_round(x, y, fill, h, radius, tint);
    } else if fill > 0 {
        fb.blend_rect(x, y, fill, h, tint);
    }
}

// A whole bar row: label cut to a third of the width, the meter across the
// middle, and the value placed from the right edge so a column of them lines up
// on its last digit. A fill narrower than the bar is squared off rather than
// rounded, since a radius wider than the fill renders as a smear and a 0.4 %
// process would draw a blob wider than its own value.
pub fn labelled(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    label: &[u8],
    value: &[u8],
    num: u64,
    den: u64,
    tint: u32,
) {
    let val_w = text::mono_width(fb, value, NUM_PX);
    let label_w = (w / 3).min(w.saturating_sub(val_w + BAR_GAP * 2));
    let top = text::centred_top(y, BAR_ROW_H, BODY_PX);
    let cut = text::fit(fb, label, BODY_PX, label_w);
    text::left(fb, x, top, cut, FOREGROUND, BODY_PX);
    text::mono_right(fb, x + w, top, value, tint, NUM_PX);
    let bar_w = w.saturating_sub(label_w + val_w + BAR_GAP * 2);
    let bar_y = y + BAR_ROW_H.saturating_sub(BAR_H) / 2;
    hbar(fb, x + label_w + BAR_GAP, bar_y, bar_w, BAR_H, num, den, tint);
}
