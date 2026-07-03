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
use alloc::string::{String, ToString};

use crate::browser::css::Computed;

use super::abs_out_of_flow::out_of_flow;
use super::collect::collect;
use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};
use super::wrap_items::wrap_items;
use super::wrap_mixed::wrap_mixed;

// Generic element: recurse into its children and pick a formatting context.
// Anchors thread their href down so links survive layout.
pub(super) fn element_box(
    w: &mut Walk,
    item: &ElementIn,
    style: Computed,
    link: &Option<String>,
    depth: u32,
) -> BoxNode {
    let tag = item.c.tag.as_str();
    let link = if tag == "a" {
        item.c.attr("href").map(|h| h.to_string()).or_else(|| link.clone())
    } else {
        link.clone()
    };
    let mut kids = collect(w.dom, item.ch, &style, w.styles, &link, depth + 1, w.count);
    // An edited textarea renders its value attribute, which the typing path
    // keeps current.
    if tag == "textarea" {
        if let Some(v) = item.c.attr("value") {
            let v = v.to_string();
            kids.clear();
            kids.push(leaf(BoxKind::Text(v), &style, &None, item.ch));
        }
    }
    // A list item leads with its marker: the ordinal in an <ol>, a bullet
    // elsewhere, unless list-style-type: none suppressed it.
    if tag == "li" && !style.list_none {
        let marker = if item.parent_tag == "ol" {
            format!("{}. ", item.ordinal)
        } else {
            String::from("\u{2022} ")
        };
        *w.count += 1;
        kids.insert(0, leaf(BoxKind::Text(marker), &style, &None, item.ch));
    }
    // An absolute box with a real inset blockifies so it stays out of inline
    // runs and the container lays it from its own list. With all insets auto
    // it keeps its natural display and flows.
    let kind = if style.is_grid {
        BoxKind::Grid
    } else if style.is_flex {
        BoxKind::Flex
    } else if style.is_block || out_of_flow(&style) {
        BoxKind::Block
    } else {
        BoxKind::Inline
    };
    let kids = match kind {
        BoxKind::Flex | BoxKind::Grid => wrap_items(&style, kids),
        BoxKind::Block => wrap_mixed(&style, kids),
        _ => kids,
    };
    BoxNode { kind, style, href: link, dom_id: item.ch, children: kids }
}
