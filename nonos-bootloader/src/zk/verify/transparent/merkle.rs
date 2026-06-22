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

use super::dir::dir;
use super::leaf::leaf;
use super::node::node;
use super::take32::take32;
use super::types::TransparentProof;

pub fn merkle_root(proof: &TransparentProof<'_>) -> Result<[u8; 32], &'static str> {
    let mut acc = leaf(proof.commitment);
    for i in 0..proof.depth as usize {
        let sibling = take32(proof.siblings, i * 32).ok_or("transparent sibling malformed")?;
        acc = if dir(proof.dirs, i)? == 0 { node(&acc, sibling) } else { node(sibling, &acc) };
    }
    Ok(acc)
}
