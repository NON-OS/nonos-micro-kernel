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

//! The Poseidon-committed FRI proof: the same shape as the BLAKE3 one, but roots
//! and Merkle paths are rate-sized field digests, so the proof can be verified
//! by an AIR.

use super::super::air::RATE;
use super::super::field::Fp;
use alloc::vec::Vec;

/// One layer's contribution to a query: the value at the queried position and at
/// its negation, each with a Poseidon Merkle path to that layer's root.
pub struct LayerOpening {
    pub a: Fp,
    pub a_path: Vec<[Fp; RATE]>,
    pub b: Fp,
    pub b_path: Vec<[Fp; RATE]>,
}

/// The openings a single query induces across every folded layer.
pub struct QueryProof {
    pub layers: Vec<LayerOpening>,
}

/// A complete Poseidon-committed FRI proof.
pub struct FriProof {
    pub roots: Vec<[Fp; RATE]>,
    pub final_layer: Vec<Fp>,
    pub queries: Vec<QueryProof>,
}
