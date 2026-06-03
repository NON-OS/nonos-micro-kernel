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

use super::layout::menubar_rect;
use super::text::draw_status;
use crate::state::indicators::clock;
use crate::state::Context;

const STATUS_ARGB: u32 = 0xFFCF_E6E9;
const GLYPH_ADVANCE: u32 = 8;
const RIGHT_PAD: u32 = 12;
const TEXT_Y_PAD: u32 = 8;
const GAP_GLYPHS: u32 = 1;

pub fn paint_status(ctx: &Context) {
    let bar = menubar_rect(ctx.width);
    let mut clk = [b' '; 5];
    let clock_label: &[u8] = if clock::hhmm(&mut clk) { &clk } else { b"--:--" };
    let battery_label: &[u8] = b"AC";
    let glyphs = battery_label.len() as u32 + GAP_GLYPHS + clock_label.len() as u32;
    let total = glyphs * GLYPH_ADVANCE;
    if bar.width <= total + RIGHT_PAD {
        return;
    }
    let y = bar.y + TEXT_Y_PAD;
    let mut x = bar.x + bar.width - RIGHT_PAD - total;
    draw_status(ctx, x, y, battery_label, STATUS_ARGB);
    x += (battery_label.len() as u32 + GAP_GLYPHS) * GLYPH_ADVANCE;
    draw_status(ctx, x, y, clock_label, STATUS_ARGB);
}
