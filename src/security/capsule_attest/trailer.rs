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

use crate::crypto::zk_kernel::EnrolledSecretProof;

use super::error::AttestError;
use super::layout::POLICY_TREE_DEPTH;

const TRAILER_MAGIC: &[u8; 8] = b"NZKCAPS2";

pub(super) fn parse(blob: &[u8]) -> Result<EnrolledSecretProof, AttestError> {
    if blob.len() < 8 || &blob[0..8] != TRAILER_MAGIC {
        return Err(AttestError::Missing);
    }
    let dir_bytes = POLICY_TREE_DEPTH.div_ceil(8);
    let expected = 137 + POLICY_TREE_DEPTH * 32 + dir_bytes;
    if blob.len() != expected || blob[136] as usize != POLICY_TREE_DEPTH {
        return Err(AttestError::Malformed);
    }
    let mut commitment = [0u8; 32];
    let mut nonce_point = [0u8; 32];
    let mut z_x = [0u8; 32];
    let mut z_r = [0u8; 32];
    commitment.copy_from_slice(&blob[8..40]);
    nonce_point.copy_from_slice(&blob[40..72]);
    z_x.copy_from_slice(&blob[72..104]);
    z_r.copy_from_slice(&blob[104..136]);
    let mut off = 137;
    let mut siblings = Vec::with_capacity(POLICY_TREE_DEPTH);
    for _ in 0..POLICY_TREE_DEPTH {
        let mut s = [0u8; 32];
        s.copy_from_slice(&blob[off..off + 32]);
        siblings.push(s);
        off += 32;
    }
    let dirs = &blob[off..off + dir_bytes];
    let mut directions = Vec::with_capacity(POLICY_TREE_DEPTH);
    for i in 0..POLICY_TREE_DEPTH {
        directions.push((dirs[i / 8] >> (i % 8)) & 1);
    }
    Ok(EnrolledSecretProof { commitment, nonce_point, z_x, z_r, siblings, directions })
}
