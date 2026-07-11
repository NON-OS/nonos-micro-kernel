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

use super::seam::Seam;
use super::types::{Note, Proof, Root};

// Client-side ZK membership prover (WASM prover in a worker, later). The stub
// returns NotWired; a live impl drives `progress` 0..100 while proving.
pub trait ProofProvider {
    // Prove `note` is in the pool under the compliant association set root.
    fn prove_membership(&self, note: &Note, assoc_set: &Root) -> Seam<Proof>;
    // Prover progress in percent for the UI, once a proof is in flight.
    fn progress(&self) -> Seam<u8>;
}
