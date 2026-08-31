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

use crate::render::palette;
use crate::render::surface::surface;
use crate::render::ui_font::scale;
use crate::state::Context;

const SHELL: u32 = palette::TEXT_DIM;
const GOOD: u32 = palette::POSITIVE;
const LOW: u32 = palette::WARN;

pub(super) fn battery_glyph(ctx: &Context, x: u32, y: u32, pct: Option<u32>) {
    let s = scale();
    let t = (3 * s + 1) / 2;
    let mut fb = surface(ctx);

    fb.stroke_round(x, y, 24 * s, 12 * s, 3 * s, t, SHELL);
    fb.fill_round(x + 24 * s, y + 4 * s, 2 * s, 4 * s, s, SHELL);

    if let Some(p) = pct {
        let room = 24 * s - (t + 2 * s) * 2;
        let fill_w = room * p.min(100) / 100;
        if fill_w > 0 {
            let color = if p > 20 { GOOD } else { LOW };
            fb.fill_round(x + t + 2 * s, y + t + 2 * s, fill_w, 12 * s - (t + 2 * s) * 2, s, color);
        }
    }
}
