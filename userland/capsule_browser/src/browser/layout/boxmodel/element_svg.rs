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

use alloc::format;
use alloc::string::String;

use crate::browser::css::{Computed, Size};

use super::attr_px::attr_px;
use super::leaf::leaf;
use super::svg_serialize::serialize_svg;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

// Box for an inline <svg>: serialize the subtree into a data URL so the same
// rasterizer that handles an <img src=*.svg> renders it. The width and height
// attributes size the box when CSS does not.
pub(super) fn element_svg(
    w: &Walk,
    item: &ElementIn,
    parent: &Computed,
    link: &Option<String>,
    style: Computed,
) -> BoxNode {
    let svg = serialize_svg(w.dom, item.ch);
    let src = format!("data:image/svg+xml,{svg}");
    let mut b = leaf(BoxKind::Image { src, alt: String::new() }, parent, link, item.ch);
    b.style = style;
    if b.style.width == Size::Auto {
        if let Some(px) = attr_px(item.c.attr("width")) {
            b.style.width = Size::Px(px);
        }
    }
    if b.style.height == Size::Auto {
        if let Some(px) = attr_px(item.c.attr("height")) {
            b.style.height = Size::Px(px);
        }
    }
    b
}
