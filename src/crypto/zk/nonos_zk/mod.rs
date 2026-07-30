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

#![allow(clippy::result_unit_err)]

mod attestation;
mod commit;
mod constants;
mod credential;
mod types;
mod zeroize;

pub use attestation::{create_attestation, verify_attestation};
pub use commit::{commit, commit_u64, verify_commitment};
pub use credential::{issue_credential, verify_credential};
pub use types::{AttestationProof, Credential};
pub use zeroize::{zeroize_array, zeroize_mut};

#[cfg(feature = "zk-halo2")]
pub mod halo2_range {
    extern crate alloc;

    use crate::crypto::zk::halo2::{halo2_verify, Halo2Error};

    pub fn verify(
        params_bytes: &[u8],
        vk_bytes: &[u8],
        proof_bytes: &[u8],
        public_inputs_columns_le32: &[&[[u8; 32]]],
    ) -> Result<(), Halo2Error> {
        halo2_verify(params_bytes, vk_bytes, proof_bytes, public_inputs_columns_le32)
    }

    pub fn single_column(inputs_le32: &[[u8; 32]]) -> [&[[u8; 32]]; 1] {
        [inputs_le32]
    }
}
