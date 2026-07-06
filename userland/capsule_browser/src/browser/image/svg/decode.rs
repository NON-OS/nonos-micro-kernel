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

use crate::browser::image::store::Decoded;

use super::affine::Affine;
use super::attr::attr;
use super::downsample::downsample;
use super::num::{num_list, parse_len};
use super::raster::Raster;
use super::state::Paint;
use super::walk::walk;
use super::xml::next_tag;

// Longest raster side; the blit scales the box's final size anyway.
const MAX_SIDE: f32 = 512.0;

pub fn is_svg(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(1024)];
    core::str::from_utf8(head).map(|s| s.contains("<svg")).unwrap_or(false)
        || head.windows(4).any(|w| w == b"<svg")
}

// Rasterize an SVG document: resolve the viewport, paint at 2x, box-filter
// down. `hint` is the largest box the page draws this image into; a vector
// stays sharp by rasterizing at that size rather than its natural one.
pub fn decode_svg(bytes: &[u8], hint: (u32, u32)) -> Option<Decoded> {
    let doc = core::str::from_utf8(bytes).ok()?;
    let mut pos = 0;
    let (root, body_at) = loop {
        let (t, n) = next_tag(doc, pos)?;
        if !t.closing && t.name == "svg" {
            break (t, n);
        }
        pos = n;
    };
    let vb = attr(root.attrs, "viewBox").map(num_list).filter(|v| v.len() >= 4);
    let attr_w = attr(root.attrs, "width").and_then(parse_len);
    let attr_h = attr(root.attrs, "height").and_then(parse_len);
    let (minx, miny, vw, vh) = match &vb {
        Some(v) if v[2] > 0.0 && v[3] > 0.0 => (v[0], v[1], v[2], v[3]),
        _ => (0.0, 0.0, attr_w.unwrap_or(300.0), attr_h.unwrap_or(150.0)),
    };
    if vw <= 0.0 || vh <= 0.0 {
        return None;
    }
    // Rasterize at the displayed size when the page told us one, keeping the
    // vector's aspect; otherwise at the natural size. Either way stay within
    // the raster ceiling.
    let natural_w = attr_w.unwrap_or(vw).max(1.0);
    let natural_h = attr_h.unwrap_or(vh).max(1.0);
    let (mut out_w, mut out_h) = if hint.0 > 0 && hint.1 > 0 {
        let scale = (hint.0 as f32 / natural_w).min(hint.1 as f32 / natural_h).max(1.0);
        (natural_w * scale, natural_h * scale)
    } else {
        (natural_w, natural_h)
    };
    let over = (out_w / MAX_SIDE).max(out_h / MAX_SIDE);
    if over > 1.0 {
        out_w /= over;
        out_h /= over;
    }
    let (ow, oh) = (out_w as u32 + 1, out_h as u32 + 1);
    let mut r = Raster::new(ow, oh);
    let t = Affine::scale(r.w as f32 / vw, r.h as f32 / vh).then(&Affine::translate(-minx, -miny));
    let paint = Paint::root(t).derive(root.attrs);
    if !root.self_closing {
        walk(doc, body_at, paint, &mut r);
    }
    Some(downsample(r, ow, oh))
}
