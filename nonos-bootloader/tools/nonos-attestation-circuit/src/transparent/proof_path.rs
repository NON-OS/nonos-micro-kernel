// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::leaf::leaf;
use super::node::node;

pub fn proof_path(commitments: &[[u8; 32]], mut index: usize) -> (Vec<[u8; 32]>, Vec<u8>) {
    let mut level: Vec<[u8; 32]> = commitments.iter().map(leaf).collect();
    let mut siblings = Vec::new();
    let mut dirs = Vec::new();
    while level.len() > 1 {
        let sibling = if index ^ 1 < level.len() { level[index ^ 1] } else { level[index] };
        siblings.push(sibling);
        dirs.push((index & 1) as u8);
        let mut next = Vec::new();
        for pair in level.chunks(2) {
            let right = if pair.len() == 2 { pair[1] } else { pair[0] };
            next.push(node(&pair[0], &right));
        }
        index /= 2;
        level = next;
    }
    (siblings, dirs)
}
