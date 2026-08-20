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

mod accumulator;
mod aggregate;
mod aggregate_build;
mod aggregate_verify;
mod attest;
mod attest_build;
mod attest_trailer;
mod attest_verify;
mod compose_check;
mod compose_witness;
mod composition;
mod copy_constraint;
mod deep_check;
mod deep_check_ext;
mod deep_terms;
mod deserialize;
mod deserialize_ext;
mod draw_ood_poseidon;
mod enroll;
mod fiat_shamir;
mod fibonacci;
mod fri_fold;
mod fused;
mod fused_ext;
mod fusion;
mod index_point;
mod measure;
mod merkle_membership;
mod multi_membership;
mod periodic_root;
mod periodic_z;
mod permutation;
mod permutation2;
mod poseidon;
mod power_chain;
mod prove;
mod prove_ext;
mod prove_ext_pre;
mod prove_poseidon_ext;
mod query_openings;
mod range_check;
mod serialize;
mod serialize_ext;
mod shared_root;
mod spec;
mod squaring;
mod trace_fold;
mod trace_fold_ext;
mod transcript_check;
mod types;
mod types_ext;
mod types_ext_pre;
mod types_poseidon_ext;
mod verify;
mod verify_ext;
mod verify_ext_pre;
mod verify_poseidon_ext;
mod wired;
mod wired_ext;
mod wired_multi_ext;

pub use accumulator::Accumulator;
pub use aggregate::{table_context, Entry, AGGREGATE_MAGIC};
pub use aggregate_build::build_aggregate;
pub use aggregate_verify::{enrolled_caps, verify_aggregate};
pub use attest::verify_membership_attestation;
pub use attest_build::{
    build_attestation_trailer, build_attestation_trailer_from_set, MeasuredSet,
};
pub use attest_trailer::verify_attestation_trailer;
pub use attest_verify::verify_membership_trailer;
pub use compose_check::{ComposeBoundary, ComposeCheck};
pub use compose_witness::{compose_inputs, compose_inputs_pub, ComposeInputs};
pub use composition::{compose, compose_ext};
pub use copy_constraint::CopyConstraint;
pub use deep_check::DeepCheck;
pub use deep_check_ext::{DeepCheckExt, DeepTerm};
pub use deep_terms::{deep_terms_query0, deep_terms_query0_pub};
pub use deserialize::deserialize_proof;
pub use deserialize_ext::deserialize_proof_ext;
pub use enroll::enroll_policy_root;
pub use fiat_shamir::FiatShamir;
pub use fibonacci::Fibonacci;
pub use fri_fold::FriFold;
pub use fused::Fused;
pub use fused_ext::FusedExt;
pub use index_point::IndexPoint;
pub use measure::measure_capsule;
pub use merkle_membership::MerkleMembership;
pub use multi_membership::{MultiMembership, Opening};
pub use periodic_root::{periodic_domain_log, periodic_root};
pub use periodic_z::PeriodicZ;
pub use permutation::Permutation;
pub use permutation2::Permutation2;
pub use poseidon::{Poseidon, NOTE_DOMAIN, NOTE_LIMBS, RATE, WIDTH};
pub use power_chain::PowerChain;
pub use prove::{stark_prove, stark_prove_bound};
pub use prove_ext::{stark_prove_ext, stark_prove_ext_blown, stark_prove_ext_blown_bound};
pub use prove_ext_pre::stark_prove_ext_preprocessed;
pub use prove_poseidon_ext::{stark_prove_poseidon_ext, stark_prove_poseidon_ext_pub};
pub use query_openings::query_openings_query0;
pub use range_check::RangeCheck;
pub use serialize::serialize_proof;
pub use serialize_ext::serialize_proof_ext;
pub use spec::{Air, AirExt};
pub use squaring::Squaring;
pub use trace_fold::TraceFold;
pub use trace_fold_ext::TraceFoldExt;
pub use transcript_check::{TranscriptCheck, TranscriptOp};
pub use types::{StarkProof, StarkQuery};
pub use types_ext::{StarkProofExt, StarkQueryExt};
pub use types_ext_pre::{PeriodicOpeningExt, StarkProofExtPre};
pub use types_poseidon_ext::{StarkProofExtP, StarkQueryExtP};
pub use verify::{stark_verify, stark_verify_bound};
pub use verify_ext::{stark_verify_ext, stark_verify_ext_blown, stark_verify_ext_blown_bound};
pub use verify_ext_pre::stark_verify_ext_preprocessed;
pub use verify_poseidon_ext::{stark_verify_poseidon_ext, stark_verify_poseidon_ext_pub};
pub use wired::Wired;
pub use wired_ext::WiredExt;
pub use wired_multi_ext::{GpGroup, WiredMultiExt};
