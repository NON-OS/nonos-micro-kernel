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
use crate::state::indicators::{clock, net};
use crate::state::Context;

const STATUS_ARGB: u32 = 0xFFCF_E6E9;
const GLYPH_ADVANCE: u32 = 8;
const RIGHT_PAD: u32 = 12;
const TEXT_Y_PAD: u32 = 8;
const GAP_GLYPHS: u32 = 2;

pub fn paint_status(ctx: &Context) {
    let bar = menubar_rect(ctx.width);
    let battery: &[u8] = b"AC";
    let network: &[u8] = if net::online() { b"NET" } else { b"OFF" };
    let mut dbuf = [b'-'; 10];
    let date: &[u8] = if clock::ymd(&mut dbuf) { &dbuf } else { b"----------" };
    let mut tbuf = [b' '; 5];
    let time: &[u8] = if clock::hhmm(&mut tbuf) { &tbuf } else { b"--:--" };
    let segments: [&[u8]; 4] = [battery, network, date, time];

    let mut glyphs = 0u32;
    for (i, s) in segments.iter().enumerate() {
        if i > 0 {
            glyphs += GAP_GLYPHS;
        }
        glyphs += s.len() as u32;
    }
    let total = glyphs * GLYPH_ADVANCE;
    if bar.width <= total + RIGHT_PAD {
        return;
    }
    let y = bar.y + TEXT_Y_PAD;
    let mut x = bar.x + bar.width - RIGHT_PAD - total;
    for (i, s) in segments.iter().enumerate() {
        if i > 0 {
            x += GAP_GLYPHS * GLYPH_ADVANCE;
        }
        draw_status(ctx, x, y, s, STATUS_ARGB);
        x += s.len() as u32 * GLYPH_ADVANCE;
    }
}
