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

//! Rebuild the depth-first row order shown in the sidebar: the children of
//! `/`, recursing into each directory that is expanded. Directories sort ahead
//! of files, each group in name order.

use alloc::string::String;
use alloc::vec::Vec;

use super::parent::parent_of;
use super::FileTree;

impl FileTree {
    pub fn rebuild_visible(&mut self) {
        self.visible.clear();
        for r in self.children_of("/") {
            self.push_subtree(r);
        }
        if self.selected >= self.visible.len() {
            self.selected = usize::MAX;
        }
    }

    pub(super) fn is_expanded(&self, path: &str) -> bool {
        self.expanded.iter().any(|e| e == path)
    }

    // Node indices directly under `parent`, directories first then files.
    fn children_of(&self, parent: &str) -> Vec<usize> {
        let mut v: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| parent_of(&n.path) == parent)
            .map(|(i, _)| i)
            .collect();
        v.sort_by(|&a, &b| {
            let (na, nb) = (&self.nodes[a], &self.nodes[b]);
            nb.is_dir.cmp(&na.is_dir).then_with(|| na.name.cmp(&nb.name))
        });
        v
    }

    fn push_subtree(&mut self, i: usize) {
        self.visible.push(i);
        let path: String = self.nodes[i].path.clone();
        if self.nodes[i].is_dir && self.is_expanded(&path) {
            for c in self.children_of(&path) {
                self.push_subtree(c);
            }
        }
    }
}
