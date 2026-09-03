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

use super::PaintBuffer;

// Sweep test without trig. A pixel's angle clockwise from 12 o'clock is
// bucketed into a quadrant by the signs of (dx, dy); inside a quadrant the
// axes are rotated into (u, v), both non-negative, so angular order is the
// order of the rational slope u/v. The fraction maps the same way to
// (boundary quadrant, remainder), leaving one cross-product comparison.
impl<'a> PaintBuffer<'a> {
    pub fn ring_arc(&mut self, cx: u32, cy: u32, r: u32, t: u32, num: u64, den: u64, argb: u32) {
        if r == 0 || t == 0 || den == 0 || num == 0 {
            return;
        }
        let num = if num > den { den } else { num };
        let turns = (num as u128) * 4;
        let bq = (turns / den as u128) as u64;
        let rem = (turns - (bq as u128) * (den as u128)) as u64;
        let outer = (r as i64) * (r as i64);
        let ir = r.saturating_sub(t) as i64;
        let inner = ir * ir;
        let x1 = cx.saturating_add(r).min(self.width.saturating_sub(1));
        let y1 = cy.saturating_add(r).min(self.height.saturating_sub(1));
        for y in cy.saturating_sub(r)..=y1 {
            for x in cx.saturating_sub(r)..=x1 {
                let dx = x as i64 - cx as i64;
                let dy = y as i64 - cy as i64;
                let d2 = dx * dx + dy * dy;
                if d2 > outer || d2 < inner {
                    continue;
                }
                let (q, u, v) = if dx >= 0 && dy < 0 {
                    (0u64, dx, -dy)
                } else if dx > 0 {
                    (1u64, dy, dx)
                } else if dy > 0 {
                    (2u64, -dx, dy)
                } else {
                    (3u64, -dy, -dx)
                };
                if q > bq {
                    continue;
                }
                let lhs = (u as u128) * ((den - rem) as u128);
                let rhs = (v as u128) * (rem as u128);
                if q == bq && lhs >= rhs {
                    continue;
                }
                self.blend_px(x, y, argb);
            }
        }
    }
}
