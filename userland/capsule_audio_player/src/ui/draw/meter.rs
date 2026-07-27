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

use crate::model::PlayerView;
use crate::ui::geometry::Rect;
use crate::ui::sprite::lerp;

use super::palette::{rgb24, ACCENT, ACCENT_DIM, GROOVE};

pub fn paint_vu(fb: &mut PaintBuffer, v: &PlayerView, r: &Rect) {
    let cols = 2u32;
    let rows = (r.h / 12).max(6);
    let cell = (r.h / rows).max(1);
    let col_w = (r.w / cols).max(1);
    let dot = (col_w.min(cell) / 2).max(3);
    let base = rows * (v.volume_q15.max(0) as u32) / 0x8000;
    for c in 0..cols {
        let lit = base.saturating_sub(c);
        for row in 0..rows {
            let dx = r.x + c * col_w;
            let dy = r.y + r.h.saturating_sub((row + 1) * cell);
            let color = if row < lit {
                let t = (row * 255 / lit.saturating_sub(1).max(1)).min(255);
                0xFF00_0000 | lerp(rgb24(ACCENT), rgb24(ACCENT_DIM), t)
            } else {
                GROOVE
            };
            fb.fill_rect(dx, dy, dot, dot, color);
        }
    }
}
