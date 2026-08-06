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

use super::tree::Dom;

impl Dom {
    /// Put `child` in front of `before` under `parent`.
    ///
    /// Appending can only ever add to the end, which is enough to build a
    /// tree once and never enough to keep one. A framework that renders a
    /// list and then reorders it moves nodes by inserting them ahead of a
    /// sibling, so without this every update after the first either lands in
    /// the wrong order or does not land at all.
    ///
    /// A `before` that is not a child of `parent` appends, which is what the
    /// specification asks for when the reference node is null.
    pub fn insert_before(&mut self, parent: usize, child: usize, before: usize) -> bool {
        if !self.attach(parent, child) {
            return false;
        }
        // `attach` put it at the end. Move it only when the reference really
        // is a sibling, and never onto itself.
        if child == before {
            return true;
        }
        let Some(at) = self.nodes[parent].children.iter().position(|&c| c == before) else {
            return true;
        };
        let Some(from) = self.nodes[parent].children.iter().rposition(|&c| c == child) else {
            return true;
        };
        if from == at {
            return true;
        }
        self.nodes[parent].children.remove(from);
        // Removing ahead of the reference shifts it down by one.
        let at = if from < at { at - 1 } else { at };
        self.nodes[parent].children.insert(at, child);
        true
    }
}
