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

use super::affine::Affine;
use super::attr::{attr, style_prop};
use super::color::parse_paint;
use super::num::parse_len;
use super::transform::parse_transform;

// Inherited paint state at one point of the element walk.
#[derive(Clone, Copy)]
pub(super) struct Paint {
    pub t: Affine,
    pub fill: Option<u32>,
    pub stroke: Option<u32>,
    pub stroke_w: f32,
    pub evenodd: bool,
}

impl Paint {
    pub fn root(t: Affine) -> Self {
        // SVG paints black by default.
        Paint { t, fill: Some(0xFF00_0000), stroke: None, stroke_w: 1.0, evenodd: false }
    }

    // This element's state: its presentation attributes and inline style
    // layered over the inherited values, its transform composed on.
    pub fn derive(&self, attrs: &str) -> Paint {
        let mut p = *self;
        if let Some(tr) = attr(attrs, "transform") {
            p.t = p.t.then(&parse_transform(tr));
        }
        let style = attr(attrs, "style").unwrap_or("");
        let prop = |name: &str| attr(attrs, name).or_else(|| style_prop(style, name));
        if let Some(v) = prop("fill") {
            p.fill = parse_paint(v);
        }
        if let Some(v) = prop("stroke") {
            p.stroke = parse_paint(v);
        }
        if let Some(v) = prop("stroke-width").and_then(parse_len) {
            p.stroke_w = v;
        }
        if let Some(v) = prop("fill-rule") {
            p.evenodd = v.trim().eq_ignore_ascii_case("evenodd");
        }
        // A single group opacity scales both paints' alpha.
        if let Some(o) = prop("opacity").and_then(parse_len) {
            let scale = o.clamp(0.0, 1.0);
            let apply = |c: u32| {
                let a = ((c >> 24) as f32 * scale) as u32;
                (a << 24) | (c & 0x00FF_FFFF)
            };
            p.fill = p.fill.map(apply);
            p.stroke = p.stroke.map(apply);
        }
        p
    }
}
