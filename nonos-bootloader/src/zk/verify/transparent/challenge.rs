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

use super::constants::DOM_ATTEST;
use super::dir::dir;
use super::take32::take32;
use super::types::TransparentProof;
use curve25519_dalek::scalar::Scalar;

pub fn challenge(
    root: &[u8; 32],
    ctx: &[u8],
    proof: &TransparentProof<'_>,
) -> Result<Scalar, &'static str> {
    let mut h = blake3::Hasher::new();
    h.update(DOM_ATTEST);
    h.update(root);
    h.update(&(ctx.len() as u64).to_le_bytes());
    h.update(ctx);
    h.update(proof.commitment);
    h.update(proof.nonce_point);
    h.update(&(proof.depth as u64).to_le_bytes());
    for i in 0..proof.depth as usize {
        h.update(take32(proof.siblings, i * 32).ok_or("transparent sibling malformed")?);
        h.update(&[dir(proof.dirs, i)?]);
    }
    Ok(Scalar::from_bytes_mod_order(*h.finalize().as_bytes()))
}
