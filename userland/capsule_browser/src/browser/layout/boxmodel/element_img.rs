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

use alloc::string::{String, ToString};

use crate::browser::css::{Computed, Size};

use super::attr_px::attr_px;
use super::img_src::img_src;
use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

// Box for an <img>. The width/height attributes are presentational hints:
// they size the box when no CSS width/height applies, so an undecoded icon
// reserves icon space, not a default box.
pub(super) fn element_img(
    w: &Walk,
    item: &ElementIn,
    parent: &Computed,
    link: &Option<String>,
    style: Computed,
) -> BoxNode {
    let src = img_src(w.dom, item.c);
    let alt = item.c.attr("alt").unwrap_or("").to_string();
    let mut b = leaf(BoxKind::Image { src, alt }, parent, link, item.ch);
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
