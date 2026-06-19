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

extern crate alloc;

use alloc::vec::Vec;

use super::super::constants::DOM_ATTEST;
use super::super::field::FieldElement;
use crate::crypto::hash::blake3_hash;

pub(super) fn challenge(
    root: &[u8; 32],
    ctx: &[u8],
    commitment: &[u8; 32],
    nonce_point: &[u8; 32],
    siblings: &[[u8; 32]],
    directions: &[u8],
) -> FieldElement {
    let mut t = Vec::with_capacity(160 + ctx.len() + siblings.len() * 33);
    t.extend_from_slice(DOM_ATTEST);
    t.extend_from_slice(root);
    t.extend_from_slice(&(ctx.len() as u64).to_le_bytes());
    t.extend_from_slice(ctx);
    t.extend_from_slice(commitment);
    t.extend_from_slice(nonce_point);
    t.extend_from_slice(&(siblings.len() as u64).to_le_bytes());
    for (sibling, direction) in siblings.iter().zip(directions.iter()) {
        t.extend_from_slice(sibling);
        t.push(*direction);
    }
    FieldElement::from_bytes(&blake3_hash(&t))
}
