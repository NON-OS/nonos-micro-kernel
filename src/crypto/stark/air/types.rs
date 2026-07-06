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

//! The STARK proof: a trace commitment, a FRI proof that the constraint
//! composition is low degree, and per-query openings that bind the composition
//! back to the committed trace.

use super::super::field::Fp;
use super::super::fri::FriProof;
use alloc::vec::Vec;

/// One consistency query: the composition value at position `p` and the trace
/// values at `p` and one row ahead, each with a Merkle path to its commitment.
pub struct StarkQuery {
    pub comp: Fp,
    pub comp_path: Vec<[u8; 32]>,
    pub t_x: Fp,
    pub t_x_path: Vec<[u8; 32]>,
    pub t_gx: Fp,
    pub t_gx_path: Vec<[u8; 32]>,
}

/// A complete STARK proof for the squaring AIR.
pub struct StarkProof {
    pub trace_root: [u8; 32],
    pub fri: FriProof,
    pub queries: Vec<StarkQuery>,
}
