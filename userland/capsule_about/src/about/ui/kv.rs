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

use crate::about::theme::{FOREGROUND, MUTED};

use super::metrics::{BODY_PX, KV_GAP, KV_LABEL_W, KV_ROW_H, NUM_PX, PAIR_LINE};
use super::text::{self, line, mono, top_of};

pub const ROW_H: u32 = KV_ROW_H;

// A label/value pair on one baseline. The label column is fixed so a stack of
// rows reads as a table without drawing one; the value is measured and cut to
// what is left, never by glyph count, because the body face is proportional.
// Numeric and hash-like values go through the mono face so digits align down
// the column. The row is placed in i32 so one that has left the pane simply does
// not land rather than being clamped to its top edge.
pub fn kv(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, label: &[u8], value: &[u8], num: bool) {
    let top = top_of(y, KV_ROW_H, BODY_PX);
    let label_w = KV_LABEL_W.min(w);
    let cut = text::fit(fb, label, BODY_PX, label_w.saturating_sub(KV_GAP));
    line(fb, x, top, cut, MUTED, BODY_PX);
    let value_x = x + label_w;
    let value_w = w.saturating_sub(label_w);
    if num {
        let cut = fit_mono(fb, value, value_w);
        mono(fb, value_x, top, cut, FOREGROUND, NUM_PX);
        return;
    }
    let cut = text::fit(fb, value, BODY_PX, value_w);
    line(fb, value_x, top, cut, FOREGROUND, BODY_PX);
}

// A label over its value. Long values that would be cut to nothing in a kv row's
// share of a narrow card get the card's whole width here instead.
pub fn pair(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, label: &[u8], value: &[u8], num: bool) {
    let cut = text::fit(fb, label, BODY_PX, w);
    line(fb, x, y, cut, MUTED, BODY_PX);
    let vy = y + PAIR_LINE as i32;
    if num {
        let cut = fit_mono(fb, value, w);
        mono(fb, x, vy, cut, FOREGROUND, NUM_PX);
        return;
    }
    let cut = text::fit(fb, value, BODY_PX, w);
    line(fb, x, vy, cut, FOREGROUND, BODY_PX);
}

// `text::fit` measures with the proportional face, which under-reports the mono
// advance and would let a hex address overrun its card. Mono cells get their own
// char-boundary-safe trim against the mono metric.
pub fn fit_mono<'a>(fb: &PaintBuffer, b: &'a [u8], max_w: u32) -> &'a [u8] {
    let s = core::str::from_utf8(b).unwrap_or("");
    let mut end = s.len();
    while end > 0 {
        if fb.measure_ttf_mono(&s[..end], NUM_PX).max(0) as u32 <= max_w {
            return &b[..end];
        }
        end -= 1;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
    }
    &b[..0]
}
