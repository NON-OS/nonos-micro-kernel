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

use super::fit_text::{fit_text, width_of};
use super::shade::elevate;
use crate::term::theme::types::Theme;

pub const RAIL_PX: f32 = 17.0;
pub const RAIL_PAD: u32 = 12;
pub const RAIL_GAP: u32 = 8;

pub fn lh() -> u32 {
    line_height(RAIL_PX).max(1) as u32
}

pub fn left(fb: &mut PaintBuffer, x: u32, y: u32, s: &str, argb: u32) -> u32 {
    fb.text_ttf(x as i32, y as i32, s, argb, RAIL_PX).max(0) as u32
}

pub fn right(fb: &mut PaintBuffer, edge: u32, y: u32, s: &str, argb: u32) {
    let w = width_of(fb, s, RAIL_PX);
    left(fb, edge.saturating_sub(w), y, s, argb);
}

pub fn clipped(fb: &mut PaintBuffer, x: u32, y: u32, max_w: u32, s: &str, argb: u32) {
    let cut = fit_text(fb, s, RAIL_PX, max_w);
    left(fb, x, y, cut, argb);
}

/// A section caption with the hairline that separates it from the block above,
/// returning the first content row so no caller re-derives the stack.
pub fn head(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, t: &Theme) -> u32 {
    fb.blend_rect(x, y, w, 1, t.chrome_edge);
    let top = y + RAIL_GAP;
    left(fb, x, top, label, t.dim);
    top + lh() + RAIL_GAP / 2
}

/// A track with the filled share drawn over it. Both are blends: the rail sits
/// on pixels the frame already painted, so a raw write would punch through.
pub fn bar(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, pct: u32, t: &Theme) {
    fb.blend_rect(x, y, w, 3, elevate(t.bg, 12));
    let fill = w * pct.min(100) / 100;
    if fill > 0 {
        fb.blend_rect(x, y, fill, 3, t.accent);
    }
}
