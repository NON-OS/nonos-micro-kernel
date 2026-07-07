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

use alloc::vec::Vec;

use crate::browser::css::Computed;

use super::field_label::field_label;
use super::leaf::leaf;
use super::tree::{BoxKind, BoxNode};
use super::walk::{ElementIn, Walk};

// Box for an <input> or <select>: a block carrying its current value or
// placeholder as a text child. Hidden inputs render nothing.
pub(super) fn element_field(w: &Walk, item: &ElementIn, style: Computed) -> Option<BoxNode> {
    if item.c.attr("type").is_some_and(|t| t.eq_ignore_ascii_case("hidden")) {
        return None;
    }
    let label = field_label(w.dom, item.ch);
    let mut kids: Vec<BoxNode> = Vec::new();
    if !label.is_empty() {
        kids.push(leaf(BoxKind::Text(label), &style, &None, item.ch));
    }
    Some(BoxNode {
        kind: BoxKind::Block,
        style,
        href: None,
        dom_id: item.ch,
        bg_image: None,
        children: kids,
    })
}
