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

use super::node::NodeKind;
use super::tree::Dom;
use super::void::is_void;

const MAX_DEPTH: u32 = 128;

impl Dom {
    /// The markup inside a node.
    ///
    /// Reading innerHTML is how a script inspects what it or the parser
    /// built, and how a great deal of code copies one part of a page into
    /// another. Only the setter existed, so reading gave undefined and the
    /// copy wrote it back out as the string "undefined".
    pub fn inner_html(&self, id: usize) -> String {
        let mut out = String::new();
        if let Some(node) = self.nodes.get(id) {
            for &c in &node.children {
                self.write_node(c, &mut out, 0);
            }
        }
        out
    }

    /// The node itself and the markup inside it.
    pub fn outer_html(&self, id: usize) -> String {
        let mut out = String::new();
        self.write_node(id, &mut out, 0);
        out
    }

    fn write_node(&self, id: usize, out: &mut String, depth: u32) {
        let Some(node) = self.nodes.get(id) else {
            return;
        };
        if node.kind != NodeKind::Element {
            escape_text(&node.text, out);
            return;
        }
        out.push('<');
        out.push_str(&node.tag);
        for (k, v) in &node.attrs {
            out.push(' ');
            out.push_str(k);
            out.push_str("=\"");
            escape_attr(v, out);
            out.push('"');
        }
        out.push('>');
        // A void element has no closing tag, and writing one would make
        // markup that reparses into a different tree than it came from.
        if is_void(&node.tag) {
            return;
        }
        if depth < MAX_DEPTH {
            for &c in &node.children {
                self.write_node(c, out, depth + 1);
            }
        }
        out.push_str("</");
        out.push_str(&node.tag);
        out.push('>');
    }
}

/// Text has to come back out as text. An unescaped `<` in a node's text
/// would reparse as the start of a tag, so a round trip through innerHTML
/// would build a tree the page never had.
fn escape_text(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
}

fn escape_attr(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '<' => out.push_str("&lt;"),
            _ => out.push(c),
        }
    }
}
