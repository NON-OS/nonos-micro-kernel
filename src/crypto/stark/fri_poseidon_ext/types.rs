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

//! The proof shape of the Poseidon-committed money-grade FRI: extension-field
//! layer openings under Poseidon Merkle roots, so the whole low-degree test can be
//! re-verified inside a STARK. The keccak `fri_ext` is the Solidity-cheap outer
//! form; this is the circuit-cheap inner form recursion folds over.

use super::super::air::RATE;
use super::super::field::{Fp, Fp2};
use alloc::vec::Vec;

/// One layer's opening at a query: the value at `i` and at `i + half`, each with a
/// Poseidon Merkle path to that layer's root.
pub struct LayerOpeningExtP {
    pub a: Fp2,
    pub a_path: Vec<[Fp; RATE]>,
    pub b: Fp2,
    pub b_path: Vec<[Fp; RATE]>,
}

/// A single query across every folded layer.
pub struct QueryProofExtP {
    pub layers: Vec<LayerOpeningExtP>,
}

/// A complete Poseidon-committed extension FRI proof.
pub struct FriProofExtP {
    pub roots: Vec<[Fp; RATE]>,
    pub final_layer: Vec<Fp2>,
    pub queries: Vec<QueryProofExtP>,
    pub pow_nonce: u64,
}
