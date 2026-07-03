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
use super::draw::draw;
use super::path::parse_path;
use super::raster::Raster;
use super::shapes::shape_polys;
use super::state::Paint;
use super::xml::next_tag;

// Subtrees that define resources or content we do not render; skipped whole
// so their geometry never paints.
fn skipped(name: &str) -> bool {
    matches!(
        name,
        "defs"
            | "symbol"
            | "clipPath"
            | "mask"
            | "style"
            | "linearGradient"
            | "radialGradient"
            | "pattern"
            | "filter"
            | "text"
            | "metadata"
            | "title"
            | "desc"
    )
}

// Walk the document from just past the root svg tag, painting shapes with
// inherited state. Groups push; unknown containers inherit silently.
pub(super) fn walk(doc: &str, from: usize, root: Paint, r: &mut Raster) {
    let mut stack: Vec<Paint> = vec![root];
    let mut pos = from;
    let mut skip = 0u32;
    while let Some((tag, next)) = next_tag(doc, pos) {
        pos = next;
        if tag.closing {
            if skip > 0 {
                skip -= 1;
            } else if matches!(tag.name, "g" | "a" | "svg" | "switch") && stack.len() > 1 {
                stack.pop();
            }
            continue;
        }
        if skip > 0 {
            if !tag.self_closing {
                skip += 1;
            }
            continue;
        }
        if skipped(tag.name) {
            if !tag.self_closing {
                skip += 1;
            }
            continue;
        }
        let cur = *stack.last().unwrap_or(&root);
        match tag.name {
            "g" | "a" | "svg" | "switch" => {
                let p = cur.derive(tag.attrs);
                if !tag.self_closing {
                    stack.push(p);
                }
            }
            "path" => {
                if let Some(d) = attr(tag.attrs, "d") {
                    draw(r, &parse_path(d), &cur.derive(tag.attrs));
                }
            }
            "rect" | "circle" | "ellipse" | "line" | "polyline" | "polygon" => {
                draw(r, &shape_polys(tag.name, tag.attrs), &cur.derive(tag.attrs));
            }
            _ => {}
        }
    }
}
