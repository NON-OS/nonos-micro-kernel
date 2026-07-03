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

use super::tree::Dom;

const MAX_ATTRS: usize = 64;

impl Dom {
    // Set or replace one attribute; names compare case-insensitively like
    // attr() reads them.
    pub fn set_attr(&mut self, id: usize, name: &str, value: String) {
        let Some(node) = self.nodes.get_mut(id) else {
            return;
        };
        for (k, v) in node.attrs.iter_mut() {
            if k.eq_ignore_ascii_case(name) {
                *v = value;
                return;
            }
        }
        if node.attrs.len() < MAX_ATTRS {
            node.attrs.push((name.to_ascii_lowercase(), value));
        }
    }
}
