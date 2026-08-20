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
use crate::security::capsule_attest::layout::POLICY_TREE_DEPTH;

const TRAILER_MAGIC: &[u8; 8] = b"NZKCAPS2";

/// The inverse of `capsule_attest::trailer::parse`, field for field. Written
/// against that reader: a trailer one byte long is refused as malformed, and
/// that looks identical to a proof that was simply wrong.
///
/// `None` rather than a blob the parser will reject.
pub fn encode(proof: &EnrolledSecretProof) -> Option<Vec<u8>> {
    if proof.siblings.len() != POLICY_TREE_DEPTH || proof.directions.len() != POLICY_TREE_DEPTH {
        return None;
    }
    let dir_bytes = POLICY_TREE_DEPTH.div_ceil(8);
    let mut out = Vec::with_capacity(137 + POLICY_TREE_DEPTH * 32 + dir_bytes);
    out.extend_from_slice(TRAILER_MAGIC);
    out.extend_from_slice(&proof.commitment);
    out.extend_from_slice(&proof.nonce_point);
    out.extend_from_slice(&proof.z_x);
    out.extend_from_slice(&proof.z_r);
    out.push(POLICY_TREE_DEPTH as u8);
    for sibling in &proof.siblings {
        out.extend_from_slice(sibling);
    }
    let mut packed = alloc::vec![0u8; dir_bytes];
    for (i, d) in proof.directions.iter().enumerate() {
        packed[i / 8] |= (d & 1) << (i % 8);
    }
    out.extend_from_slice(&packed);
    Some(out)
}
