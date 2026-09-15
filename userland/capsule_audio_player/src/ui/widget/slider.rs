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

//! Scrubber and volume slider. The painter and the hit-test share `permille`,
//! so a click always lands on the value the groove was drawn at.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::metrics::pill;
use crate::ui::paint::{disc, fill};
use crate::ui::theme::{alpha, mix, rgb, CYAN, CYAN_DIM, DEEP, INK};

pub fn permille(r: Rect, x: i32) -> u32 {
    if r.w <= 0 {
        return 0;
    }
    (((x - r.x) as i64 * 1000 / r.w as i64).clamp(0, 1000)) as u32
}

fn groove(r: Rect, th: i32) -> Rect {
    Rect::new(r.x, r.cy() - th / 2, r.w, th)
}

pub fn slider(fb: &mut PaintBuffer, r: Rect, num: u64, den: u64, thumb: bool) {
    let th = (r.h * 38 / 100).max(4);
    let g = groove(r, th);
    fill(fb, g, pill(th), DEEP);
    let filled = if den == 0 { 0 } else { (r.w as u64 * num.min(den) / den) as i32 };
    if filled > 0 {
        let steps = filled.min(48).max(1);
        for i in 0..steps {
            let sx = filled * i / steps;
            let sw = (filled * (i + 1) / steps - sx).max(1);
            let c = alpha(mix(rgb(CYAN_DIM), rgb(CYAN), (i * 255 / steps) as u32), 0xFF);
            fill(fb, Rect::new(g.x + sx, g.y, sw, th), 0, c);
        }
        fill(fb, Rect::new(g.x, g.y, filled.min(th), th), pill(th), CYAN_DIM);
    }
    if thumb {
        disc(fb, r.x + filled, r.cy(), (r.h * 40 / 100).max(5), INK);
    }
}
