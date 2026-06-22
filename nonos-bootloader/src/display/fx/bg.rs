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

use super::color::mix;

const BG: u32 = 0xFF04070B;
const GLOW: u32 = 0xFF00F5D4;
const BLACK: u32 = 0xFF000000;

// Atmosphere color at a pixel: base, a soft cyan lift behind the upper
// wordmark, an edge vignette, and scanline texture. Integer-only and pure,
// so it can be written once with no framebuffer reads.
pub(super) fn bg_at(x: u32, y: u32, w: u32, h: u32) -> u32 {
    let mut c = BG;

    let cx = (w / 2) as i64;
    let nx = (x as i64 - cx) * 1000 / 540;
    let ny = (y as i64 - 150) * 1000 / 360;
    let d2 = nx * nx + ny * ny;
    if d2 < 1_000_000 {
        c = mix(c, GLOW, ((1_000_000 - d2) * 26 / 1_000_000) as u32);
    }

    let mcx = (w / 2) as i64;
    let mcy = (h / 2) as i64;
    let maxd2 = (mcx * mcx + mcy * mcy).max(1);
    let vd2 = (x as i64 - mcx).pow(2) + (y as i64 - mcy).pow(2);
    let r = vd2 * 1000 / maxd2;
    if r > 302 {
        c = mix(c, BLACK, (((r - 302) * 120 / 698) as u32).min(120));
    }

    if y % 3 == 0 {
        c = mix(c, BLACK, 16);
    }
    c
}
