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


use super::layout::Rect;
use super::surface::surface;
use super::ui_font::scale;
use crate::state::Context;

pub fn round_fill(ctx: &Context, r: Rect, radius_logical: u32, argb: u32) {
    let rad = radius_logical * scale();
    surface(ctx).fill_round(r.x, r.y, r.width, r.height, rad, argb);
}

pub fn panel(ctx: &Context, r: Rect, radius_logical: u32, fill: u32, border: u32) {
    let rad = radius_logical * scale();
    surface(ctx).panel(r.x, r.y, r.width, r.height, rad, fill, border);
}

pub fn shadow_panel(ctx: &Context, r: Rect, radius_logical: u32, fill: u32, border: u32) {
    let rad = radius_logical * scale();
    let spread = 3 * scale();
    let mut fb = surface(ctx);
    fb.shadow_round(r.x, r.y, r.width, r.height, rad, spread, SHADOW);
    fb.panel(r.x, r.y, r.width, r.height, rad, fill, border);
}

pub fn blend(ctx: &Context, r: Rect, argb: u32) {
    surface(ctx).blend_rect(r.x, r.y, r.width, r.height, argb);
}

const SHADOW: u32 = 0x5A00_0000;
