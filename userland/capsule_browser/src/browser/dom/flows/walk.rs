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
use crate::browser::dom::node::NodeKind;
use crate::browser::dom::tree::Dom;
use crate::browser::html::flow::{Flow, Style};
use crate::browser::html::parse::flush::flush;

use super::append_text::append_text;
use super::enter_element::enter_element;
use super::leave_element::leave_element;

const MAX_DEPTH: u32 = 400;

pub fn walk(dom: &Dom, id: usize, mut style: Style, link: Option<String>, out: &mut Vec<Flow>, buf: &mut String, depth: u32, styles: &[Computed]) {
    if depth > MAX_DEPTH {
        return;
    }
    let node = &dom.nodes[id];
    match node.kind {
        NodeKind::Text => return append_text(out, buf, &node.text, style, &link),
        NodeKind::Document => {
            for &c in &node.children {
                walk(dom, c, style, link.clone(), out, buf, depth + 1, styles);
            }
            return;
        }
        NodeKind::Element => {}
    }
    if styles.get(id).is_some_and(|c| c.display_none) {
        return;
    }
    let name = node.tag.as_str();
    if matches!(name, "script" | "style" | "head" | "title" | "noscript" | "template") {
        return;
    }
    flush(out, buf, style, &link);
    if let Some(cs) = styles.get(id) {
        style.color = cs.color;
        style.bg = cs.bg;
        style.bold = style.bold || cs.bold;
    }
    let mut link = link;
    enter_element(out, node, &mut style, &mut link);
    for &c in &node.children {
        walk(dom, c, style, link.clone(), out, buf, depth + 1, styles);
    }
    flush(out, buf, style, &link);
    leave_element(out, name);
}
