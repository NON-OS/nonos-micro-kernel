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

//! The decoded-peak waveform. Buckets left of the playhead take the cyan
//! gradient, buckets right of it the muted ground, so the bar doubles as a
//! progress read-out without a second control.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::metrics::S1;
use crate::ui::paint::fill;
use crate::ui::theme::{mix, rgb, CYAN, CYAN_DIM, MUTE};

const BAR: i32 = 5;

pub fn waveform(fb: &mut PaintBuffer, r: Rect, buckets: &[u8], num: u64, den: u64) {
    if buckets.is_empty() || r.w <= 0 {
        return;
    }
    let step = BAR + S1 / 2;
    let n = (r.w / step).max(1);
    let head = if den == 0 { 0 } else { (num * n as u64 / den) as i32 };
    for i in 0..n {
        let b = buckets[(i as usize * buckets.len() / n as usize).min(buckets.len() - 1)];
        let h = ((b as i32 * r.h / 255).max(2) / 2) * 2;
        let bar = Rect::new(r.x + i * step, r.cy() - h / 2, BAR, h);
        let c = if i <= head {
            0xFF00_0000 | mix(rgb(CYAN_DIM), rgb(CYAN), (i * 255 / n) as u32)
        } else {
            MUTE
        };
        fill(fb, bar, 2, c);
    }
}
