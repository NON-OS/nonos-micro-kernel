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

/// `text_ttf` takes the top of the line box, so a row that wants its text
/// optically centred has to subtract the line height itself. Painter and hit
/// test both come through here, which is what keeps a click on the same pixels
/// the label was drawn on.
pub fn centred_top(row_y: u32, row_h: u32, px: f32) -> i32 {
    let lh = line_height(px).max(1) as u32;
    row_y as i32 + ((row_h.saturating_sub(lh)) / 2) as i32
}

pub fn left(fb: &mut PaintBuffer, x: u32, top: i32, s: &str, argb: u32, px: f32) -> i32 {
    fb.text_ttf(x as i32, top, s, argb, px)
}

pub fn right(fb: &mut PaintBuffer, right_x: u32, top: i32, s: &str, argb: u32, px: f32) -> u32 {
    let w = fb.measure_ttf(s, px).max(0) as u32;
    let x = right_x.saturating_sub(w);
    fb.text_ttf(x as i32, top, s, argb, px);
    x
}

pub fn width(fb: &PaintBuffer, s: &str, px: f32) -> u32 {
    fb.measure_ttf(s, px).max(0) as u32
}
