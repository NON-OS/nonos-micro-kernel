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

use alloc::vec;
use alloc::vec::Vec;

use super::attr::attr;
use super::math::{cos, sin, PI};
use super::num::num_list;

type P = [f32; 2];

const ELLIPSE_STEPS: u32 = 48;

fn a(attrs: &str, name: &str) -> f32 {
    attr(attrs, name).and_then(|v| super::num::parse_len(v)).unwrap_or(0.0)
}

// Basic shape elements as closed/open polylines in user coordinates.
// Rounded rect corners are drawn square: a visible simplification that keeps
// the silhouette.
pub(super) fn shape_polys(tag: &str, attrs: &str) -> Vec<Vec<P>> {
    match tag {
        "rect" => {
            let (x, y) = (a(attrs, "x"), a(attrs, "y"));
            let (w, h) = (a(attrs, "width"), a(attrs, "height"));
            if w <= 0.0 || h <= 0.0 {
                return Vec::new();
            }
            vec![vec![[x, y], [x + w, y], [x + w, y + h], [x, y + h], [x, y]]]
        }
        "circle" | "ellipse" => {
            let (cx, cy) = (a(attrs, "cx"), a(attrs, "cy"));
            let r = a(attrs, "r");
            let (rx, ry) = if tag == "circle" { (r, r) } else { (a(attrs, "rx"), a(attrs, "ry")) };
            if rx <= 0.0 || ry <= 0.0 {
                return Vec::new();
            }
            let mut pts = Vec::with_capacity(ELLIPSE_STEPS as usize + 1);
            for i in 0..=ELLIPSE_STEPS {
                let t = 2.0 * PI * i as f32 / ELLIPSE_STEPS as f32;
                pts.push([cx + rx * cos(t), cy + ry * sin(t)]);
            }
            vec![pts]
        }
        "line" => {
            vec![vec![[a(attrs, "x1"), a(attrs, "y1")], [a(attrs, "x2"), a(attrs, "y2")]]]
        }
        "polyline" | "polygon" => {
            let n = num_list(attr(attrs, "points").unwrap_or(""));
            let mut pts: Vec<P> = n.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
            if pts.len() < 2 {
                return Vec::new();
            }
            if tag == "polygon" {
                let first = pts[0];
                pts.push(first);
            }
            vec![pts]
        }
        _ => Vec::new(),
    }
}
