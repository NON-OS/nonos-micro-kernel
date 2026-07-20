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

//! Implied end tags. Real HTML routinely omits close tags for list items, table
//! cells, paragraphs and the like; opening one of these closes the open element
//! it may not nest inside, so the tree still nests the way a browser would build
//! it rather than piling the omitted-close elements into ever-deeper children.

use super::tree::Dom;

// Walk up from `cur` popping every open element that opening `opening` implies
// the end of, and return the element the new node should attach under.
pub fn auto_close(dom: &Dom, cur: usize, opening: &str) -> usize {
    let mut n = cur;
    while n != 0 && closes(opening, dom.nodes[n].tag.as_str()) {
        n = dom.nodes[n].parent;
    }
    n
}

// Whether opening `opening` implicitly closes an open `current`.
fn closes(opening: &str, current: &str) -> bool {
    match opening {
        "li" => current == "li",
        "dt" | "dd" => current == "dt" || current == "dd",
        "option" => current == "option",
        "td" | "th" => current == "td" || current == "th",
        "tr" => matches!(current, "tr" | "td" | "th"),
        "thead" | "tbody" | "tfoot" => matches!(current, "tr" | "td" | "th"),
        // A block-level element cannot sit inside a paragraph, so it ends one.
        _ => is_block(opening) && current == "p",
    }
}

// The block-level elements that end an open paragraph when they appear.
fn is_block(tag: &str) -> bool {
    matches!(
        tag,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "div"
            | "dl"
            | "fieldset"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hr"
            | "main"
            | "nav"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "ul"
    )
}
