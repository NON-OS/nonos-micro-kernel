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

use super::FileTree;

impl FileTree {
    // Act on the visible row at `vis`: toggle a directory (returns None), or
    // return the path of a file to open.
    pub fn activate(&mut self, vis: usize) -> Option<String> {
        if vis >= self.visible.len() {
            return None;
        }
        self.selected = vis;
        let ni = self.visible[vis];
        if self.nodes[ni].is_dir {
            let path = self.nodes[ni].path.clone();
            match self.expanded.iter().position(|e| *e == path) {
                Some(pos) => {
                    self.expanded.remove(pos);
                }
                None => self.expanded.push(path),
            }
            self.rebuild_visible();
            None
        } else {
            Some(self.nodes[ni].path.clone())
        }
    }
}
