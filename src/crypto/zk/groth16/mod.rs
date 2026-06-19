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

#![cfg(feature = "zk-groth16")]

pub mod attestation_vk;
pub mod deserialize;
pub mod error;
pub mod params;
mod read_proof_bls12_381;
mod verifier;
mod verifier_bls12_381;

pub use attestation_vk::verify_attestation;
pub use error::Groth16Error;
pub use params::{MAX_PROOF_BYTES, MAX_PUBLIC_INPUTS, MAX_VK_BYTES};
pub use verifier::{groth16_verify_bn254, Groth16Verifier};
pub use verifier_bls12_381::groth16_verify_bls12_381;
