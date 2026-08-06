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

use crate::browser::html::parse::read_to_gt::read_to_gt;
use crate::browser::html::parse::tag_name::tag_name;

use super::attrs::parse_attrs;
use super::auto_close::auto_close;
use super::close_tag::close_tag;
use super::comment::skip_comment;
use super::node::NodeKind;
use super::raw_text::raw_text;
use super::tree::Dom;
use super::void::is_void;

pub fn consume(
    dom: &mut Dom,
    cur: usize,
    chars: &mut core::iter::Peekable<core::str::CharIndices>,
) -> usize {
    // Before anything else, because a comment does not end where a tag would.
    if skip_comment(chars) {
        return cur;
    }
    let raw = read_to_gt(chars);
    if raw.starts_with('!') || raw.starts_with('?') {
        return cur;
    }
    let name = tag_name(&raw);
    if name.is_empty() {
        return cur;
    }
    if raw.starts_with('/') {
        return close_tag(dom, cur, &name);
    }
    // Close any open element this one implicitly ends (an open <li> before a new
    // <li>, an open <p> before a block, an open <td> before the next cell) so the
    // omitted end tags common in real HTML still nest correctly.
    let cur = auto_close(dom, cur, &name);
    let Some(id) = dom.push(cur, NodeKind::Element, name.clone()) else {
        return cur;
    };
    dom.nodes[id].attrs = parse_attrs(&raw);
    if matches!(name.as_str(), "script" | "style" | "title" | "textarea") {
        let txt = raw_text(chars, &name);
        let Some(t) = dom.push(id, NodeKind::Text, String::new()) else {
            return cur;
        };
        dom.nodes[t].text = txt;
        cur
    } else if is_void(&name) {
        cur
    } else {
        id
    }
}
