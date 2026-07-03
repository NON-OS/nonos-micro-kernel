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

use crate::browser::css::Computed;
use crate::browser::dom::Dom;

use super::body_id::body_id;
use super::collect::collect;
use super::tree::{BoxKind, BoxNode};
use super::wrap_mixed::wrap_mixed;

// Box tree for the page: rooted at <body> (or the document when a page has
// none), display:none subtrees dropped, mixed children wrapped.
pub fn build(dom: &Dom, styles: &[Computed]) -> BoxNode {
    let root_id = body_id(dom);
    let style = styles.get(root_id).copied().unwrap_or_else(Computed::root);
    let mut count = 0usize;
    let children = collect(dom, root_id, &style, styles, &None, 0, &mut count);
    BoxNode {
        kind: BoxKind::Block,
        style,
        href: None,
        dom_id: root_id,
        children: wrap_mixed(&style, children),
    }
}
