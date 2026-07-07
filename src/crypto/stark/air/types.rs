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

//! The STARK proof in its DEEP form: commitments to the trace columns and the
//! constraint composition, the trace evaluations at an out-of-domain point (the
//! OOD frame), a FRI proof that the DEEP quotient polynomial is low degree, and
//! per-query openings binding that quotient to the committed trace and
//! composition.

use super::super::field::Fp;
use super::super::fri::FriProof;
use alloc::vec::Vec;

/// One consistency query at position `p`: the DEEP polynomial value (opened
/// against the FRI layer-zero commitment), each trace column at `p`, and the
/// composition at `p`, each with a Merkle path to its commitment.
pub struct StarkQuery {
    pub deep: Fp,
    pub deep_path: Vec<[u8; 32]>,
    pub trace: Vec<Fp>,
    pub trace_paths: Vec<Vec<[u8; 32]>>,
    pub comp: Fp,
    pub comp_path: Vec<[u8; 32]>,
}

/// A complete STARK proof.
pub struct StarkProof {
    pub trace_roots: Vec<[u8; 32]>,
    pub comp_root: [u8; 32],
    /// The trace columns evaluated at `g^k * z` for each window row `k`, laid
    /// out row-major like a transition window: `ood_frame[k * width + col]`.
    pub ood_frame: Vec<Fp>,
    pub fri: FriProof,
    pub queries: Vec<StarkQuery>,
}
