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

use nonos_toolkit::decorations::{frame_rect, radius, BORDER_PX, FRAME_BORDER, TRANSPARENT};
use nonos_toolkit::paint::radius::{clamp_radius, coverage};

use crate::paint::PaintBuffer;

pub(super) fn finish(fb: &mut PaintBuffer, maximized: bool) {
    let r = radius(maximized);
    if r == 0 {
        return;
    }
    let f = frame_rect(fb.width, fb.height, maximized);
    let r = clamp_radius(f.w, f.h, r);
    if r == 0 {
        return;
    }
    let stride = fb.stride_words;
    let len = fb.pixels.len();
    for py in f.h.saturating_sub(r)..f.h {
        for px in 0..f.w {
            if px >= r && px < f.w.saturating_sub(r) {
                continue;
            }
            if coverage(px, py, f.w, f.h, r) != 0 {
                continue;
            }
            let idx = ((f.y + py) * stride + f.x + px) as usize;
            if idx < len {
                fb.pixels[idx] = TRANSPARENT;
            }
        }
    }
    fb.stroke_round(f.x, f.y, f.w, f.h, r, BORDER_PX, FRAME_BORDER);
}
