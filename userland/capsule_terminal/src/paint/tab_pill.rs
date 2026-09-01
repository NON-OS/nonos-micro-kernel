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

use super::fit_text::{fit_text, width_of};
use super::tokens::{DOT_IDLE, DOT_OK, TAB_ACTIVE, TAB_ACTIVE_EDGE, TAB_IDLE};
use super::tokens::{TOOLBAR_ACTIVE, TOOLBAR_LABEL};
use crate::layout::Rect;

pub const PILL_W: u32 = 132;
pub const PLUS_W: u32 = 28;
pub const PILL_H: u32 = 26;
pub const LABEL_PX: f32 = 13.0;
pub const RADIUS: u32 = 7;

const GAP: u32 = 4;
const CLOSE_W: u32 = 18;
const DOT_X: u32 = 11;
const TEXT_X: u32 = 20;

/// Rect of tab `i` inside an accessory offering `avail_w` px to tabs and the
/// new-tab chip. A zero width means the tab does not fit and must not be drawn
/// or hit-tested.
pub fn pill_rect(i: usize, avail_w: u32) -> Rect {
    let x = i as u32 * PILL_W;
    let fits = x + PILL_W + PLUS_W <= avail_w;
    Rect { x, y: 0, w: if fits { PILL_W - GAP } else { 0 }, h: PILL_H }
}

/// Close-affordance sub-rect at the trailing edge of `pill`.
pub fn close_rect(pill: Rect) -> Rect {
    let w = CLOSE_W.min(pill.w);
    Rect { x: pill.x + pill.w - w, y: pill.y, w, h: pill.h }
}

/// Rect of the new-tab chip that follows `n` tabs.
pub fn plus_rect(n: usize, avail_w: u32) -> Rect {
    let x = (n as u32 * PILL_W).min(avail_w.saturating_sub(PLUS_W));
    Rect { x, y: 0, w: PLUS_W - GAP, h: PILL_H }
}

pub fn draw_pill(fb: &mut PaintBuffer, r: Rect, label: &[u8], active: bool) {
    if r.w == 0 {
        return;
    }
    fb.fill_round(r.x, r.y, r.w, r.h, RADIUS, if active { TAB_ACTIVE } else { TAB_IDLE });
    if active {
        fb.stroke_round(r.x, r.y, r.w, r.h, RADIUS, 1, TAB_ACTIVE_EDGE);
    }
    fb.circle(r.x + DOT_X, r.y + r.h / 2, 3, if active { DOT_OK } else { DOT_IDLE });
    let fg = if active { TOOLBAR_ACTIVE } else { TOOLBAR_LABEL };
    let baseline = (r.y + r.h / 2) as i32 - (LABEL_PX as i32) / 2;
    let close = close_rect(r);
    let cut = core::str::from_utf8(label).unwrap_or("");
    let cut = fit_text(fb, cut, LABEL_PX, close.x.saturating_sub(r.x + TEXT_X));
    fb.text_ttf((r.x + TEXT_X) as i32, baseline, cut, fg, LABEL_PX);
    let cx = close.x + close.w / 2 - width_of(fb, "x", LABEL_PX) / 2;
    fb.text_ttf(cx as i32, baseline, "x", fg, LABEL_PX);
}
