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

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::css::Computed;

use super::element_box::element_box;
use super::element_field::element_field;
use super::element_img::element_img;
use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

// Per-tag dispatch for one element child: non-rendered subtrees drop, <br>,
// <img> and form fields are leaves, everything else recurses.
pub(super) fn element(
    w: &mut Walk,
    item: &ElementIn,
    parent: &Computed,
    link: &Option<String>,
    depth: u32,
    out: &mut Vec<BoxNode>,
) {
    let mut style =
        w.styles.get(item.ch).copied().unwrap_or_else(|| Computed::inherit_from(parent));
    if style.display_none {
        return;
    }
    let tag = item.c.tag.as_str();
    // The hidden attribute, a closed <dialog> and an unopened popover are
    // user-agent-hidden until script toggles them.
    if item.c.attr("hidden").is_some()
        || item.c.attr("popover").is_some()
        || (tag == "dialog" && item.c.attr("open").is_none())
    {
        return;
    }
    // User-agent table roles by tag, so <table>/<tr>/<td> lay out as a column grid
    // without the page having to declare `display: table*`. All three are
    // block-level so the box tree keeps rows and cells as clean block children.
    match tag {
        "table" => {
            style.is_table = true;
            style.is_block = true;
        }
        "tr" => {
            style.is_table_row = true;
            style.is_block = true;
        }
        "td" | "th" => {
            style.is_table_cell = true;
            style.is_block = true;
        }
        _ => {}
    }
    if matches!(tag, "script" | "style" | "head" | "title" | "noscript" | "template") {
        return;
    }
    *w.count += 1;
    if tag == "br" {
        // Hard line break: a newline the inline layout honors.
        out.push(leaf(BoxKind::Text(String::from("\n")), parent, link, item.ch));
        return;
    }
    if tag == "img" {
        out.push(element_img(w, item, parent, link, style));
        return;
    }
    if tag == "svg" {
        out.push(super::element_svg::element_svg(w, item, parent, link, style));
        return;
    }
    if matches!(tag, "input" | "select") {
        if let Some(b) = element_field(w, item, style) {
            out.push(b);
        }
        return;
    }
    out.push(element_box(w, item, style, link, depth));
}
