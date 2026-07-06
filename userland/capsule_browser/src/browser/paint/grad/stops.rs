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

// The interpolated ARGB color at position t along the stop list. Before the
// first stop and after the last the ends hold, matching CSS clamping.
pub(super) fn color_at(stops: &[(u32, f32)], t: f32) -> u32 {
    let first = stops[0];
    if t <= first.1 {
        return first.0;
    }
    let last = stops[stops.len() - 1];
    if t >= last.1 {
        return last.0;
    }
    for w in stops.windows(2) {
        let (c0, p0) = w[0];
        let (c1, p1) = w[1];
        if t >= p0 && t <= p1 {
            let f = if p1 > p0 { (t - p0) / (p1 - p0) } else { 0.0 };
            return lerp(c0, c1, f);
        }
    }
    last.0
}

fn lerp(a: u32, b: u32, f: f32) -> u32 {
    let mut out = 0u32;
    for s in [0, 8, 16, 24] {
        let x = ((a >> s) & 0xff) as f32;
        let y = ((b >> s) & 0xff) as f32;
        out |= ((x + (y - x) * f) as u32 & 0xff) << s;
    }
    out
}
