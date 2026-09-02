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

use super::spark_smooth::smooth;
use crate::layout::Rect;
use crate::rail::ring::SPARK_SAMPLES;

const DOT_R: u32 = 2;
const AREA_ALPHA: u32 = 0x20_00_00_00;

/// A trend line over the CPU window with the area beneath it tinted and the
/// newest sample marked. One point has no shape and a flat line would read as
/// a real idle stretch, so the plot stays empty until two can be joined.
///
/// The plotted series is the smoothed one: the tint is a light wash under a
/// curve rather than a comb of raw columns, and the anti-aliased trace over it
/// is what the eye actually follows. The ring keeps its unsmoothed readings.
pub fn draw_spark(fb: &mut PaintBuffer, r: Rect, data: &[u8], head: usize, colour: u32) {
    let mut curve = [0u8; SPARK_SAMPLES];
    let n = smooth(&data[..data.len().min(SPARK_SAMPLES)], head, &mut curve);
    if n < 2 || r.w < 2 || r.h < 2 {
        return;
    }
    let fill = (colour & 0x00FF_FFFF) | AREA_ALPHA;
    let mut pts = [(0i32, 0i32); SPARK_SAMPLES];
    for i in 0..n {
        let px = col(r.x, r.w, i, n);
        let v = (curve[i].min(100) as u32) * (r.h - 1) / 100;
        let py = r.y + r.h - 1 - v;
        let next = if i + 1 < n { col(r.x, r.w, i + 1, n) } else { px + 1 };
        fb.blend_rect(px, py, (next - px).max(1), (r.y + r.h).saturating_sub(py), fill);
        pts[i] = (px as i32, py as i32);
    }
    fb.polyline_aa(&pts[..n], colour);
    let (dx, dy) = pts[n - 1];
    fb.circle(dx as u32, dy as u32, DOT_R, colour);
}

fn col(x: u32, w: u32, i: usize, n: usize) -> u32 {
    x + (i as u32 * (w - 1)) / (n as u32 - 1)
}
