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

use super::palette::{ACCENT, GROOVE, MUTED};

pub fn paint_vu(fb: &mut PaintBuffer, v: &PlayerView, r: &Rect) {
    let cols = 2u32;
    let rows = (r.h / 12).max(6);
    let cell = (r.h / rows).max(1);
    let col_w = (r.w / cols).max(1);
    let dot = col_w.min(cell).saturating_sub(3).max(3);
    let base = rows * (v.volume_q15.max(0) as u32) / 0x8000;
    for c in 0..cols {
        let lit = base.saturating_sub(c);
        for row in 0..rows {
            let dx = r.x + c * col_w;
            let dy = r.y + r.h.saturating_sub((row + 1) * cell);
            let color = if row < lit { ACCENT } else if row * 3 >= rows * 2 { MUTED } else { GROOVE };
            fb.fill_rect(dx, dy, dot, dot, color);
        }
    }
}
