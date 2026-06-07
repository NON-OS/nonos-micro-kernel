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

// The BLS12-381 Groth16 verifying key for NONOS attestation, embedded in the
// kernel image so a capsule's attestation proof can be checked at spawn with no
// external file or network. This is the same verifying key the bootloader uses
// for the kernel proof, so kernel and capsule attestation share one trust root.

use crate::crypto::zk::groth16::{groth16_verify_bls12_381, Groth16Error};

pub static ATTESTATION_VK: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/nonos-bootloader/tools/nonos-attestation-circuit/generated_keys/attestation_verifying_key.bin"
));

// Verify an attestation proof (kernel or capsule) against the embedded
// verifying key. `public_inputs_be` is the flat 32-byte big-endian field
// element layout the prover emits. Ok(()) only when the proof is valid.
#[must_use = "attestation result must be checked before trusting the artifact"]
pub fn verify_attestation(proof_blob: &[u8], public_inputs_be: &[u8]) -> Result<(), Groth16Error> {
    groth16_verify_bls12_381(ATTESTATION_VK, proof_blob, public_inputs_be)
}
