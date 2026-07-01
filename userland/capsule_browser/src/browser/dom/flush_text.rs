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

pub fn flush_text(dom: &mut Dom, parent: usize, buf: &mut String) {
    if buf.is_empty() {
        return;
    }
    let Some(t) = dom.push(parent, NodeKind::Text, String::new()) else {
        buf.clear();
        return;
    };
    dom.nodes[t].text = core::mem::take(buf);
}
