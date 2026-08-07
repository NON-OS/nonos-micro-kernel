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
use super::pool_provider::PoolProvider;
use super::proof_provider::ProofProvider;
use super::quote_provider::QuoteProvider;
use super::revenue_provider::RevenueProvider;
use super::sim_provider::SimProvider;
use super::stub::Stub;

// The one place the active backend is chosen. Screens call these, never a
// concrete impl, so wiring the contracts+prover means swapping the body here and
// nothing else. Today every one returns the honest Stub.
pub fn pool() -> impl PoolProvider {
    Stub
}
pub fn prover() -> impl ProofProvider {
    Stub
}
pub fn quote() -> impl QuoteProvider {
    Stub
}
pub fn assoc() -> impl AssocSetProvider {
    Stub
}
pub fn sim() -> impl SimProvider {
    Stub
}
pub fn revenue() -> impl RevenueProvider {
    Stub
}
