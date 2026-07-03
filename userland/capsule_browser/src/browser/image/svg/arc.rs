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

use super::math::{abs, atan2, cos, sin, sqrt, PI};

const ARC_STEP_RAD: f32 = 0.2;

type P = [f32; 2];

// Elliptical arc via the endpoint-to-center conversion of SVG F.6.5, sampled
// at a fixed angular step. Degenerate radii collapse to a line.
pub(super) fn arc_to(
    out: &mut Vec<P>,
    p0: P,
    radii: P,
    rot_deg: f32,
    large: bool,
    sweep: bool,
    p1: P,
) {
    let (mut rx, mut ry) = (abs(radii[0]), abs(radii[1]));
    if rx < 1e-6 || ry < 1e-6 {
        out.push(p1);
        return;
    }
    let phi = rot_deg * PI / 180.0;
    let (cp, sp) = (cos(phi), sin(phi));
    let dx = (p0[0] - p1[0]) / 2.0;
    let dy = (p0[1] - p1[1]) / 2.0;
    let x1 = cp * dx + sp * dy;
    let y1 = -sp * dx + cp * dy;
    let lam = x1 * x1 / (rx * rx) + y1 * y1 / (ry * ry);
    if lam > 1.0 {
        let s = sqrt(lam);
        rx *= s;
        ry *= s;
    }
    let num = rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1;
    let den = rx * rx * y1 * y1 + ry * ry * x1 * x1;
    let mut co = if den > 0.0 && num > 0.0 { sqrt(num / den) } else { 0.0 };
    if large == sweep {
        co = -co;
    }
    let cx1 = co * rx * y1 / ry;
    let cy1 = -co * ry * x1 / rx;
    let cx = cp * cx1 - sp * cy1 + (p0[0] + p1[0]) / 2.0;
    let cy = sp * cx1 + cp * cy1 + (p0[1] + p1[1]) / 2.0;
    let a0 = atan2((y1 - cy1) / ry, (x1 - cx1) / rx);
    let a1 = atan2((-y1 - cy1) / ry, (-x1 - cx1) / rx);
    let mut sweep_ang = a1 - a0;
    if sweep && sweep_ang < 0.0 {
        sweep_ang += 2.0 * PI;
    } else if !sweep && sweep_ang > 0.0 {
        sweep_ang -= 2.0 * PI;
    }
    let steps = ((abs(sweep_ang) / ARC_STEP_RAD) as u32).clamp(2, 64);
    for i in 1..=steps {
        let a = a0 + sweep_ang * i as f32 / steps as f32;
        out.push([
            cp * rx * cos(a) - sp * ry * sin(a) + cx,
            sp * rx * cos(a) + cp * ry * sin(a) + cy,
        ]);
    }
}
