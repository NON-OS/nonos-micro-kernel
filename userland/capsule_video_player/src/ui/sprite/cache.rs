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

//! Masks are supersampled once on first use and box-filtered down at blit time,
//! so a frame pays only for the destination pixels it actually touches.

extern crate alloc;

use alloc::vec::Vec;
use nonos_app_skeleton::paint::PaintBuffer;
use spin::Once;

use super::canvas::Sprite;
use super::downscale::alpha;
use super::kind::Glyph;
use super::table::BUILDERS;

const SRC: u32 = 64;
const WHITE: u32 = 0x00FF_FFFF;

static MASKS: Once<Vec<Sprite>> = Once::new();

fn masks() -> &'static Vec<Sprite> {
    MASKS.call_once(|| BUILDERS.iter().map(|b| b(SRC, WHITE)).collect())
}

pub fn draw(fb: &mut PaintBuffer, x: u32, y: u32, n: u32, argb: u32, g: Glyph) {
    let a = (argb >> 24) & 0xFF;
    if n == 0 || a == 0 {
        return;
    }
    let m = &masks()[g as usize];
    let rgb = argb & 0x00FF_FFFF;
    for dy in 0..n {
        for dx in 0..n {
            let sa = alpha(m, dx, dy, n);
            if sa != 0 {
                fb.blend_px(x + dx, y + dy, ((sa * a / 255) << 24) | rgb);
            }
        }
    }
}
