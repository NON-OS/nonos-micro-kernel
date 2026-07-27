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

//! The preprocessed-periodic root: the keccak wide-periodic Merkle root over the
//! coset low-degree extension of an AIR's periodic columns. It is the constant a
//! preprocessed-periodic verifier bakes, and the value a per-program verifier key
//! binds to the program that produced the wiring. The preprocessed prover
//! (`prove_ext_pre`) commits the periodic tree through the same function here, so a
//! root computed for registration and a root committed inside a proof are the same
//! object by construction, not by agreement.

use alloc::vec::Vec;

use super::super::field::Fp;
use super::super::fri::root_of_unity;
use super::super::merkle::MerkleTree;
use super::super::poly::lde;
use super::composition::domain_params_blown;
use super::spec::AirExt;

// The coset shift the whole prover uses. It must match `prove_ext_pre`'s.
const SHIFT: u64 = 7;

/// The coset extension of the periodic columns and the wide-periodic tree over it.
/// Both the preprocessed prover and the root helper go through this, so the periodic
/// domain size, the coset, and the leaf and node rules cannot drift between them.
pub(super) fn periodic_lde_tree<A: AirExt>(
    air: &A,
    extra_blowup_bits: u32,
) -> (Vec<Vec<Fp>>, MerkleTree) {
    let log_t = air.log_trace_len();
    let (log_n, _) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);
    // The per-column coset extension is independent, so the 444-column LDE — the
    // dominant cost of a deep-domain emit — parallelizes over columns; the tree
    // then commits with the parallel leaf layer. Serial and parallel produce the
    // identical root by construction.
    let cols = air.periodic_columns();
    let periodic_d: Vec<Vec<Fp>> = crate::par::map_slice(&cols, |col| lde(col, g, shift, omega, n));
    let tree = MerkleTree::commit_wide_periodic(&periodic_d);
    (periodic_d, tree)
}

/// The 32-byte preprocessed-periodic root for `air` at the given FRI rate. This is
/// the value a per-program verifier key binds; the preprocessed prover commits the
/// identical tree, so the two never diverge.
pub fn periodic_root<A: AirExt>(air: &A, extra_blowup_bits: u32) -> [u8; 32] {
    periodic_lde_tree(air, extra_blowup_bits).1.root()
}

/// The log2 of the periodic evaluation domain for `air` at this FRI rate: the
/// size each periodic column is extended to. Deterministic public structure,
/// exposed so an out-of-crate committer (for example a parallel one) sizes the
/// domain identically to the serial path and cannot drift from it.
pub fn periodic_domain_log<A: AirExt>(air: &A, extra_blowup_bits: u32) -> u32 {
    domain_params_blown(air, extra_blowup_bits).0
}
