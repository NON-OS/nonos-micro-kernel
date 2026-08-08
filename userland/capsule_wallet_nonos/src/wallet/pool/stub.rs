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

use super::assoc_set_provider::AssocSetProvider;
use super::decoded::{DecodedTx, StateDiff};
use super::pool_provider::PoolProvider;
use super::proof_provider::ProofProvider;
use super::quote_provider::QuoteProvider;
use super::revenue_provider::RevenueProvider;
use super::seam::{Inclusion, Seam};
use super::sim_provider::SimProvider;
use super::types::{Commitment, Fees, Note, Proof, Root, TxRef};
use crate::wallet::swap::{Reserves, Token};

// The wiring is not connected yet. Every call returns NotWired so the UI renders
// the honest "not yet connected" state. Replace with live impls to go live; no
// UI change is required.
pub struct Stub;

impl PoolProvider for Stub {
    fn deposit_note(&self, _amount_wei: u128) -> Seam<Commitment> {
        Seam::NotWired
    }
    fn roots(&self) -> Seam<Root> {
        Seam::NotWired
    }
    fn submit_withdraw(&self, _proof: &Proof) -> Seam<TxRef> {
        Seam::NotWired
    }
    fn fees(&self) -> Seam<Fees> {
        Seam::NotWired
    }
}

impl ProofProvider for Stub {
    fn prove_membership(&self, _note: &Note, _assoc_set: &Root) -> Seam<Proof> {
        Seam::NotWired
    }
    fn progress(&self) -> Seam<u8> {
        Seam::NotWired
    }
}

impl AssocSetProvider for Stub {
    fn current_root(&self) -> Seam<Root> {
        Seam::NotWired
    }
    fn inclusion(&self, _note: &Note) -> Inclusion {
        Inclusion::NotWired
    }
}

impl SimProvider for Stub {
    fn simulate(&self, _tx: &DecodedTx) -> Seam<StateDiff> {
        Seam::NotWired
    }
}

impl RevenueProvider for Stub {
    fn fee_bps(&self) -> Seam<u32> {
        Seam::NotWired
    }
    fn cumulative_revenue_wei(&self) -> Seam<[u8; 32]> {
        Seam::NotWired
    }
    fn staking_apr_bps(&self) -> Seam<u32> {
        Seam::NotWired
    }
    fn staked_wei(&self) -> Seam<[u8; 32]> {
        Seam::NotWired
    }
}

impl QuoteProvider for Stub {
    fn reserves(&self, _pay: Token, _receive: Token) -> Seam<Reserves> {
        Seam::NotWired
    }
    fn gas(&self, _pay: Token, _receive: Token) -> Seam<u64> {
        Seam::NotWired
    }
}
