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

use crate::pm::state::{Ring, SAMPLES};

use super::metrics::SPARK_DOT_R;

pub fn cpu(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, ring: &Ring, tint: u32) {
    plot(fb, x, y, w, h, |i| ring.cpu_at(i) as u32, ring.len(), 100, tint);
}

// Memory has no natural ceiling, so the tallest sample in the window sets the
// scale; the shape is a trend against its own peak, never against a made-up max.
pub fn mem(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, ring: &Ring, tint: u32) {
    let hi = (0..ring.len()).map(|i| ring.mem_at(i)).max().unwrap_or(0);
    plot(fb, x, y, w, h, |i| ring.mem_at(i), ring.len(), hi, tint);
}

fn col(x: u32, w: u32, i: usize, n: usize) -> u32 {
    x + (i as u32 * (w - 1)) / (n as u32 - 1)
}

// A single sample has no shape: a flat line would read as a real idle stretch,
// so the plot stays empty until two points can be joined.
fn plot(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    at: impl Fn(usize) -> u32,
    n: usize,
    hi: u32,
    tint: u32,
) {
    if n < 2 || w < 2 || h < 2 || hi == 0 {
        return;
    }
    let n = n.min(SAMPLES);
    let fill = (tint & 0x00FF_FFFF) | 0x3800_0000;
    let mut pts = [(0i32, 0i32); SAMPLES];
    for i in 0..n {
        let px = col(x, w, i, n);
        let v = at(i).min(hi) as u64 * (h - 1) as u64 / hi as u64;
        let py = y + h - 1 - v as u32;
        let next = if i + 1 < n { col(x, w, i + 1, n) } else { px + 1 };
        fb.blend_rect(px, py, (next - px).max(1), (y + h).saturating_sub(py), fill);
        pts[i] = (px as i32, py as i32);
    }
    fb.polyline_aa(&pts[..n], tint);
    let (dx, dy) = pts[n - 1];
    fb.circle(dx as u32, dy as u32, SPARK_DOT_R, tint);
}
