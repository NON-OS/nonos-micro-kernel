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

pub fn set_text_content(dom: &mut Dom, id: usize, s: String) {
    if id >= dom.nodes.len() {
        return;
    }
    dom.nodes[id].children.clear();
    if let Some(t) = dom.push(id, NodeKind::Text, String::new()) {
        dom.nodes[t].text = s;
    }
}
