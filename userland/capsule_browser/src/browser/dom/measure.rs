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

use alloc::vec;

use super::tree::Dom;

impl Dom {
    /// Record where each node was laid out, from the rectangles just painted.
    ///
    /// A node can paint more than one rectangle: an inline that wraps leaves
    /// one fragment per line. What a script asks for is the box around the
    /// whole element, so the fragments belonging to a node are unioned rather
    /// than the last one winning, which would report only the final line.
    pub fn record_rects<'a, I>(&mut self, frags: I)
    where
        I: Iterator<Item = (usize, i32, i32, i32, i32)> + 'a,
    {
        self.rects = vec![[0; 4]; self.nodes.len()];
        let mut seen = vec![false; self.nodes.len()];
        for (node, x, y, w, h) in frags {
            // Anonymous boxes carry node 0, which is the document itself and
            // never something a script measures.
            if node == 0 || node >= self.rects.len() {
                continue;
            }
            if !seen[node] {
                self.rects[node] = [x, y, w, h];
                seen[node] = true;
                continue;
            }
            let cur = self.rects[node];
            let (left, top) = (cur[0].min(x), cur[1].min(y));
            let right = (cur[0] + cur[2]).max(x + w);
            let bottom = (cur[1] + cur[3]).max(y + h);
            self.rects[node] = [left, top, right - left, bottom - top];
        }
    }

    /// One number from a node's box: 0 is x, 1 is y, 2 is width, 3 is height.
    ///
    /// A node that was never laid out reports zero, which is what a browser
    /// reports for one that is not displayed.
    pub fn box_of(&self, node: usize, which: usize) -> i32 {
        match self.rects.get(node) {
            Some(r) => r.get(which).copied().unwrap_or(0),
            None => 0,
        }
    }
}
