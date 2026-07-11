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
use super::types::{Commitment, Fees, Proof, Root, TxRef};

// The NOX Privacy Pool contract surface. Implemented later against the deployed
// pool on chain_id 1; the Stub returns NotWired so the UI stays honest.
pub trait PoolProvider {
    // Commit a deposit note of `amount_wei`; returns the on-chain commitment.
    fn deposit_note(&self, amount_wei: u128) -> Seam<Commitment>;
    // Current pool merkle root.
    fn roots(&self) -> Seam<Root>;
    // Submit a withdraw with a membership proof; returns the tx reference.
    fn submit_withdraw(&self, proof: &Proof) -> Seam<TxRef>;
    // Live fee breakdown for a withdraw/swap.
    fn fees(&self) -> Seam<Fees>;
}
