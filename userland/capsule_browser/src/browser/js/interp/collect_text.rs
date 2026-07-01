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

pub fn collect_text(dom: &Dom, id: usize, out: &mut String, depth: u32) {
    if depth > 400 || out.len() >= 100_000 || id >= dom.nodes.len() {
        return;
    }
    let n = &dom.nodes[id];
    if n.kind == NodeKind::Text {
        out.push_str(&n.text);
    }
    for &c in &n.children {
        collect_text(dom, c, out, depth + 1);
    }
}
