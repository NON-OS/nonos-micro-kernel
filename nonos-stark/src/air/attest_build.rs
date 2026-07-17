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

//! Building a capsule's attestation trailer: the prover side an enrollment tool
//! runs. Measure the images to the policy root, prove membership of one image bound
//! to its context at money-grade soundness, and pack the magic, path, and proof in
//! the exact byte layout the kernel spawn gate parses. The root is not shipped; the
//! kernel holds its own, and enrollment publishes the same one.

use super::super::field::Fp;
use super::super::poseidon_merkle::PoseidonMerkleTree;
use super::measure::measure_capsule;
use super::poseidon::{Poseidon, RATE};
use super::prove_ext::stark_prove_ext_blown_bound;
use super::serialize_ext::serialize_proof_ext;
use super::MerkleMembership;
use alloc::vec::Vec;

const MAGIC: &[u8; 8] = b"NZKSTRK1";

/// The attestation trailer for `images[index]`, bound to `context`, at the given
/// soundness. Layout: magic, depth, the sibling path (four little-endian words per
/// node), the direction bits, then the serialized proof.
#[allow(clippy::too_many_arguments)]
pub fn build_attestation_trailer(
    hasher: &Poseidon,
    log_rounds: u32,
    images: &[&[u8]],
    index: usize,
    context: &[u8],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> Vec<u8> {
    let leaves: Vec<[Fp; RATE]> = images.iter().map(|i| measure_capsule(hasher, i)).collect();
    let tree = PoseidonMerkleTree::commit(hasher, &leaves);
    let path = tree.open(index);
    let depth = path.len();
    let directions: Vec<bool> = (0..depth).map(|k| (index >> k) & 1 == 1).collect();

    let air =
        MerkleMembership::new(hasher.clone(), log_rounds, tree.root(), path.clone(), directions);
    let trace = air.trace(leaves[index]);
    let proof = stark_prove_ext_blown_bound(&air, &trace, n_queries, grind_bits, extra_blowup_bits, context);

    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.push(depth as u8);
    for node in &path {
        for lane in node {
            out.extend_from_slice(&lane.value().to_le_bytes());
        }
    }
    let mut dirs = alloc::vec![0u8; depth.div_ceil(8)];
    for (i, k) in (0..depth).enumerate() {
        if (index >> k) & 1 == 1 {
            dirs[i / 8] |= 1 << (i % 8);
        }
    }
    out.extend_from_slice(&dirs);
    out.extend_from_slice(&serialize_proof_ext(&proof));
    out
}
