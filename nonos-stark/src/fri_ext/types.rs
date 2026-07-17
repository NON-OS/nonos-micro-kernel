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

//! The money-grade FRI proof: extension-field layers, plus a proof-of-work nonce
//! bound before the query positions. The layers are `Fp2` because the folds are
//! drawn from the extension.

use super::super::field::Fp2;
use alloc::vec::Vec;

/// One layer's contribution to a query: the extension value at the queried
/// position and at its negation, each with a Merkle path to that layer's root.
pub struct LayerOpeningExt {
    pub a: Fp2,
    pub a_path: Vec<[u8; 32]>,
    pub b: Fp2,
    pub b_path: Vec<[u8; 32]>,
}

/// The openings a single query induces across every folded layer.
pub struct QueryProofExt {
    pub layers: Vec<LayerOpeningExt>,
}

/// A complete money-grade FRI proof: extension-field challenges give ~2^-128
/// folding soundness, and the grinding nonce adds proof-of-work to the queries.
pub struct FriProofExt {
    pub roots: Vec<[u8; 32]>,
    pub final_layer: Vec<Fp2>,
    pub queries: Vec<QueryProofExt>,
    pub pow_nonce: u64,
}
