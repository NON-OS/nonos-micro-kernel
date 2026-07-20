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

//! The FRI proof: commitment roots, the small final layer sent in full, and the
//! opened query positions that bind consecutive layers together.

use super::super::field::Fp;
use alloc::vec::Vec;

/// One layer's contribution to a query: the value at the queried position `i`
/// and at its negation `i + n/2`, each with a Merkle path to that layer's root.
pub struct LayerOpening {
    pub a: Fp,
    pub a_path: Vec<[u8; 32]>,
    pub b: Fp,
    pub b_path: Vec<[u8; 32]>,
}

/// The openings a single query induces across every folded layer.
pub struct QueryProof {
    pub layers: Vec<LayerOpening>,
}

/// A complete FRI low-degree proof.
pub struct FriProof {
    pub roots: Vec<[u8; 32]>,
    pub final_layer: Vec<Fp>,
    pub queries: Vec<QueryProof>,
}
