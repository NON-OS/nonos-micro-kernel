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

//! Backend seam. Every not-yet-wired call (privacy pool, ZK prover, association
//! set, simulation, NOX revenue) sits behind a trait here. `Stub` returns a
//! typed NotWired/Pending outcome so the UI is honest; live impls drop in with
//! no UI change. The seam contract is documented in the capsule README.

mod active;
mod assoc_set_provider;
mod decoded;
mod pool_provider;
mod proof_provider;
mod quote_provider;
mod revenue_provider;
mod seam;
mod sim_provider;
mod stub;
mod types;

pub use active::{assoc, pool, prover, quote, revenue, sim};
pub use assoc_set_provider::AssocSetProvider;
pub use decoded::{BalanceDelta, DecodedTx, StateDiff};
pub use pool_provider::PoolProvider;
pub use proof_provider::ProofProvider;
pub use quote_provider::QuoteProvider;
pub use revenue_provider::RevenueProvider;
pub use seam::{Inclusion, Seam};
pub use sim_provider::SimProvider;
pub use stub::Stub;
pub use types::{Commitment, Fees, Note, Proof, Root, TxRef};
