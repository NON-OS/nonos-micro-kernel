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

//! An end-to-end STARK over an algebraic intermediate representation. A trace is
//! interpolated and extended onto an evaluation coset, its constraints become a
//! composition polynomial, FRI proves that composition is low degree, and query
//! openings bind it to the committed trace. The AIR is a trait, so one engine
//! proves any computation expressed as a trace with transition and boundary
//! constraints. This closes the transparent, post-quantum proof system: the
//! verifier is proven against forgeries, not assumed sound.

mod composition;
mod copy_constraint;
mod fiat_shamir;
mod fibonacci;
mod fri_fold;
mod fused;
mod lookup;
mod merkle_membership;
mod multi_membership;
mod permutation;
mod permutation2;
mod poseidon;
mod poseidon_preimage;
mod power_chain;
mod prove;
mod recursive_verifier;
mod spec;
mod squaring;
mod trace_fold;
mod types;
mod verify;
mod wired;

pub use copy_constraint::CopyConstraint;
pub use fiat_shamir::FiatShamir;
pub use fibonacci::Fibonacci;
pub use fri_fold::FriFold;
pub use fused::Fused;
pub use lookup::{Lookup, TupleLookup};
pub use merkle_membership::MerkleMembership;
pub use multi_membership::{MultiMembership, Opening};
pub use permutation::Permutation;
pub use permutation2::Permutation2;
pub use poseidon::{Poseidon, RATE, WIDTH};
pub use poseidon_preimage::{poseidon_preimage_trace, PoseidonPreimage};
pub use power_chain::PowerChain;
pub use prove::stark_prove;
pub use recursive_verifier::{
    layer_membership, layer_openings, recursive_verify, FinalLayerConstant, FriTranscript,
};
pub use spec::Air;
pub use squaring::Squaring;
pub use trace_fold::TraceFold;
pub use types::{StarkProof, StarkQuery};
pub use verify::stark_verify;
pub use wired::Wired;
