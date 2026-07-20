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

//! The preprocessed-periodic proof: a money-grade proof plus the periodic
//! sidecar a deployment verifier consumes instead of recomputing the
//! structural columns. The claims at z are transcript-absorbed before the
//! DEEP coefficients are drawn, and each consistency query carries the wide
//! periodic row with one path against the baked periodic commitment.

use super::super::field::{Fp, Fp2};
use super::types_ext::StarkProofExt;
use alloc::vec::Vec;

/// One consistency query's periodic opening: every periodic-column value at
/// the query row, authenticated by a single wide-leaf path.
pub struct PeriodicOpeningExt {
    pub row: Vec<Fp>,
    pub path: Vec<[u8; 32]>,
}

/// A money-grade proof with the periodic sidecar. `openings` parallels the
/// proof's consistency queries in order.
pub struct StarkProofExtPre {
    pub proof: StarkProofExt,
    /// The claimed periodic-column evaluations at the out-of-domain point.
    pub periodic_z: Vec<Fp2>,
    pub openings: Vec<PeriodicOpeningExt>,
}
