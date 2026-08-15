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

//! A battery outline filled in proportion to the real charge. Green when
//! healthy, amber when low; on AC power the caller draws no fill.

use crate::render::fill::fill_rect;
use crate::render::ui_font::scale;
use crate::state::Context;

const SHELL: u32 = 0xFFB8_C6D6;
const GOOD: u32 = 0xFF5F_D08A;
const LOW: u32 = 0xFFE0_9A5A;

pub(super) fn battery_glyph(ctx: &Context, x: u32, y: u32, pct: Option<u32>) {
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let bar =
        |gx: u32, gy: u32, gw: u32, gh: u32, c: u32| fill_rect(va, st, w, h, gx, gy, gw, gh, c);

    let s = scale();

    // Shell: a 20x11 outline with a small terminal nub on the right.
    bar(x, y, 20 * s, s, SHELL);
    bar(x, y + 10 * s, 20 * s, s, SHELL);
    bar(x, y, s, 11 * s, SHELL);
    bar(x + 19 * s, y, s, 11 * s, SHELL);
    bar(x + 20 * s, y + 3 * s, 2 * s, 5 * s, SHELL);

    if let Some(p) = pct {
        let p = p.min(100);
        let fill_w = 18 * s * p / 100;
        if fill_w > 0 {
            let color = if p > 20 { GOOD } else { LOW };
            bar(x + s, y + 2 * s, fill_w, 7 * s, color);
        }
    }
}
