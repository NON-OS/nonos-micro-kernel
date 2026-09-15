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

//! The i32 facade over `nonos_toolkit::paint`. Every rect the screens draw is
//! signed and may sit partly outside the surface, so each entry point clips to
//! the buffer before handing whole-pixel coordinates down.

use nonos_app_skeleton::{measure_ttf, PaintBuffer};

use super::geometry::Rect;
use super::metrics::cap_h;

fn clip(fb: &PaintBuffer, r: Rect) -> Option<(u32, u32, u32, u32)> {
    let x0 = r.x.max(0);
    let y0 = r.y.max(0);
    let x1 = r.right().min(fb.width as i32);
    let y1 = r.bottom().min(fb.height as i32);
    if x1 <= x0 || y1 <= y0 {
        return None;
    }
    Some((x0 as u32, y0 as u32, (x1 - x0) as u32, (y1 - y0) as u32))
}

pub fn fill(fb: &mut PaintBuffer, r: Rect, rad: u32, argb: u32) {
    if let Some((x, y, w, h)) = clip(fb, r) {
        fb.fill_round(x, y, w, h, rad, argb);
    }
}

pub fn stroke(fb: &mut PaintBuffer, r: Rect, rad: u32, t: u32, argb: u32) {
    if let Some((x, y, w, h)) = clip(fb, r) {
        fb.stroke_round(x, y, w, h, rad, t, argb);
    }
}

pub fn panel(fb: &mut PaintBuffer, r: Rect, rad: u32, fill_argb: u32, border: u32) {
    fill(fb, r, rad, fill_argb);
    stroke(fb, r, rad, 1, border);
}

pub fn shadow(fb: &mut PaintBuffer, r: Rect, rad: u32, spread: u32, argb: u32) {
    if let Some((x, y, w, h)) = clip(fb, r) {
        fb.shadow_round(x, y, w, h, rad, spread, argb);
    }
}

pub fn disc(fb: &mut PaintBuffer, cx: i32, cy: i32, rad: i32, argb: u32) {
    if cx - rad >= 0 && cy - rad >= 0 && rad > 0 {
        fb.circle(cx as u32, cy as u32, rad as u32, argb);
    }
}

pub fn ring(fb: &mut PaintBuffer, cx: i32, cy: i32, rad: i32, t: i32, argb: u32) {
    if cx - rad >= 0 && cy - rad >= 0 && rad > 0 {
        fb.ring(cx as u32, cy as u32, rad as u32, t.max(1) as u32, argb);
    }
}

pub fn width(s: &str, px: f32) -> i32 {
    measure_ttf(s, px)
}

pub fn text(fb: &mut PaintBuffer, x: i32, top: i32, s: &str, argb: u32, px: f32) -> i32 {
    fb.text_ttf(x, top, s, argb, px)
}

pub fn text_mid(fb: &mut PaintBuffer, r: Rect, s: &str, argb: u32, px: f32) -> i32 {
    fb.text_ttf(r.x, r.cy() - cap_h(px), s, argb, px)
}

pub fn text_centre(fb: &mut PaintBuffer, r: Rect, s: &str, argb: u32, px: f32) -> i32 {
    let x = r.x + (r.w - width(s, px)) / 2;
    fb.text_ttf(x, r.cy() - cap_h(px), s, argb, px)
}

pub fn text_right(fb: &mut PaintBuffer, r: Rect, s: &str, argb: u32, px: f32) -> i32 {
    let x = r.right() - width(s, px);
    fb.text_ttf(x, r.cy() - cap_h(px), s, argb, px)
}
