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

use crate::browser::dom;

use super::copy_children::copy_children;
use super::ctx::Ctx;

// innerHTML setter: parse the fragment with the page parser and graft the
// result under the target, replacing its children.
pub(super) fn graft_html(ctx: &mut Ctx, id: usize, html: &str) {
    if id >= ctx.dom.nodes.len() {
        return;
    }
    let frag = dom::parse(html.as_bytes());
    ctx.dom.nodes[id].children.clear();
    copy_children(ctx.dom, &frag, 0, id, 0);
    ctx.dirty = true;
}
