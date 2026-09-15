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

//! The four-bar equalizer that replaces the index on a playing row.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::paint::fill;
use crate::ui::theme::CYAN;

const PHASE: [i32; 4] = [88, 46, 100, 62];

pub fn equalizer(fb: &mut PaintBuffer, r: Rect, running: bool, step: u32) {
    let bw = (r.w / 7).max(2);
    let gap = (r.w - bw * 4) / 3;
    for (i, base) in PHASE.iter().enumerate() {
        let pct = if running {
            let t = (step + i as u32 * 37) % 100;
            let swing = if t < 50 { t } else { 100 - t };
            (*base * (55 + swing as i32 * 90 / 100)) / 100
        } else {
            *base / 3
        };
        let h = (r.h * pct.clamp(12, 100) / 100).max(2);
        let x = r.x + i as i32 * (bw + gap);
        fill(fb, Rect::new(x, r.bottom() - h, bw, h), 1, CYAN);
    }
}
