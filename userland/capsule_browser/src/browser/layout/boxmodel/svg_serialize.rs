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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

const MAX_DEPTH: u32 = 40;

// Serialize a DOM subtree back to SVG markup so an inline <svg> can be handed
// to the same rasterizer that decodes an <img src=*.svg>. The svg namespace
// is asserted on the root so a fragment lifted from HTML stands alone.
pub(super) fn serialize_svg(dom: &Dom, id: usize) -> String {
    let mut out = String::new();
    write_node(dom, id, &mut out, 0, true);
    out
}

fn write_node(dom: &Dom, id: usize, out: &mut String, depth: u32, root: bool) {
    if depth > MAX_DEPTH {
        return;
    }
    let Some(node) = dom.nodes.get(id) else { return };
    match node.kind {
        NodeKind::Text => out.push_str(node.text.trim()),
        NodeKind::Element => {
            out.push('<');
            out.push_str(&node.tag);
            if root && node.attr("xmlns").is_none() {
                out.push_str(" xmlns=\"http://www.w3.org/2000/svg\"");
            }
            for (k, v) in &node.attrs {
                out.push(' ');
                out.push_str(k);
                out.push_str("=\"");
                out.push_str(v);
                out.push('"');
            }
            out.push('>');
            for &ch in &node.children {
                write_node(dom, ch, out, depth + 1, false);
            }
            out.push_str("</");
            out.push_str(&node.tag);
            out.push('>');
        }
        NodeKind::Document => {}
    }
}
