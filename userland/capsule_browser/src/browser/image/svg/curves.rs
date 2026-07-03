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

use alloc::vec::Vec;

const CURVE_STEPS: u32 = 16;

type P = [f32; 2];

// Fixed-step flattening; at icon scale sixteen chords sit within the
// supersampled pixel grid.
pub(super) fn cubic_to(out: &mut Vec<P>, p0: P, c1: P, c2: P, p1: P) {
    for i in 1..=CURVE_STEPS {
        let t = i as f32 / CURVE_STEPS as f32;
        let u = 1.0 - t;
        let x = u * u * u * p0[0]
            + 3.0 * u * u * t * c1[0]
            + 3.0 * u * t * t * c2[0]
            + t * t * t * p1[0];
        let y = u * u * u * p0[1]
            + 3.0 * u * u * t * c1[1]
            + 3.0 * u * t * t * c2[1]
            + t * t * t * p1[1];
        out.push([x, y]);
    }
}

pub(super) fn quad_to(out: &mut Vec<P>, p0: P, c: P, p1: P) {
    for i in 1..=CURVE_STEPS {
        let t = i as f32 / CURVE_STEPS as f32;
        let u = 1.0 - t;
        let x = u * u * p0[0] + 2.0 * u * t * c[0] + t * t * p1[0];
        let y = u * u * p0[1] + 2.0 * u * t * c[1] + t * t * p1[1];
        out.push([x, y]);
    }
}
