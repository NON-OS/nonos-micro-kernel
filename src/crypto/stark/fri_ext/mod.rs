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

//! The money-grade FRI low-degree test: identical in structure to `fri`, but the
//! fold challenges are drawn from the degree-2 extension `Fp2` and a proof-of-work
//! nonce is ground before the queries. Extension challenges take the folding
//! soundness error from ~2^-64 to ~2^-128, and grinding raises the query
//! soundness. This is the FRI a high-value proof uses; the base `fri` module
//! remains for the recursive-verifier arithmetization.

mod prove;
mod types;
mod verify;

pub use prove::fri_prove_ext;
pub use types::{FriProofExt, LayerOpeningExt, QueryProofExt};
pub use verify::fri_verify_ext;
