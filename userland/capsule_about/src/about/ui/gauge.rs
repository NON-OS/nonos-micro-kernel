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
use nonos_toolkit::font::ttf::line_height;

use crate::about::theme::{ACCENT, LABEL, MUTED, TITLE, TRACK_BG};

use super::card;
use super::metrics::{
    BODY_PX, CARD_PAD, TILE_H, TILE_RING_R, TILE_RING_T, TILE_RING_TOP, TILE_SUB_GAP, VALUE_PX,
};
use super::text;

pub const HEIGHT: u32 = TILE_H;

// A ring, the value it stands for, and one line of provenance under it. The
// arc and the track share a centre and a radius, so the filled sweep is always
// exactly the fraction `num/den` of the ring the eye compares it against.
pub fn tile(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    label: &[u8],
    value: &[u8],
    sub: &[u8],
    num: u64,
    den: u64,
) {
    card::panel(fb, x, y as i32, w, TILE_H);
    let cx = x + w / 2;
    let cy = y + TILE_RING_TOP + TILE_RING_R;
    fb.ring(cx, cy, TILE_RING_R, TILE_RING_T, TRACK_BG);
    fb.ring_arc(cx, cy, TILE_RING_R, TILE_RING_T, num, den.max(1), ACCENT);
    let value_top = text::top_of((cy - TILE_RING_R) as i32, TILE_RING_R * 2, VALUE_PX);
    let value_w = text::width(fb, value, VALUE_PX);
    text::line(fb, cx.saturating_sub(value_w / 2), value_top, value, TITLE, VALUE_PX);
    let inner = card::inner(w);
    let label_y = cy + TILE_RING_R + CARD_PAD;
    label_line(fb, x, label_y, inner, label, LABEL);
    let step = line_height(BODY_PX).max(1) as u32 + TILE_SUB_GAP;
    label_line(fb, x, label_y + step, inner, sub, MUTED);
}

// Both lines under the ring are centred on the card, which means measuring the
// cut string rather than the original: a truncated label centres on what is
// actually drawn.
fn label_line(fb: &mut PaintBuffer, x: u32, y: u32, inner: u32, bytes: &[u8], argb: u32) {
    let cut = text::fit(fb, bytes, BODY_PX, inner);
    let w = text::width(fb, cut, BODY_PX);
    text::line(fb, x + CARD_PAD + inner.saturating_sub(w) / 2, y as i32, cut, argb, BODY_PX);
}
