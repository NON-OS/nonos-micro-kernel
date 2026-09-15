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

//! The two-stop rounded ground every cover sits on, plus the bottom scrim.
//! Rows inside the corner band fill opaque through `blend_rect`; only the
//! `rad` rows at each end pay for per-pixel corner coverage.

use nonos_app_skeleton::paint::radius::coverage;
use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::theme::{alpha, mix, SCRIM};

fn row_span(fb: &mut PaintBuffer, r: Rect, rad: u32, row: i32, argb: u32) {
    let (w, h) = (r.w as u32, r.h as u32);
    if row >= rad as i32 && (row as u32) < h.saturating_sub(rad) {
        blend(fb, r.x, r.y + row, r.w, argb);
        return;
    }
    let a = (argb >> 24) & 0xFF;
    for col in 0..r.w {
        let cov = coverage(col as u32, row as u32, w, h, rad);
        if cov == 0 {
            continue;
        }
        let px = r.x + col;
        if px >= 0 && r.y + row >= 0 {
            fb.blend_px(px as u32, (r.y + row) as u32, ((a * cov / 255) << 24) | (argb & 0x00FF_FFFF));
        }
    }
}

fn blend(fb: &mut PaintBuffer, x: i32, y: i32, w: i32, argb: u32) {
    if y < 0 || x + w <= 0 {
        return;
    }
    let x0 = x.max(0);
    fb.blend_rect(x0 as u32, y as u32, (x + w - x0).max(0) as u32, 1, argb);
}

pub fn ground(fb: &mut PaintBuffer, r: Rect, rad: u32, top: u32, bottom: u32) {
    if r.w <= 0 || r.h <= 0 {
        return;
    }
    for row in 0..r.h {
        let t = (row as u32 * 255) / r.h as u32;
        row_span(fb, r, rad, row, alpha(mix(top, bottom, t), 0xFF));
    }
}

pub fn scrim(fb: &mut PaintBuffer, r: Rect, rad: u32) {
    let start = r.h * 45 / 100;
    let span = (r.h - start).max(1);
    for row in start..r.h {
        let a = ((row - start) as u32 * (SCRIM >> 24) / span as u32) as u8;
        if a > 0 {
            row_span(fb, r, rad, row, alpha(0, a));
        }
    }
}
