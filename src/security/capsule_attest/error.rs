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

// Reasons a capsule attestation can be rejected. Every variant is a refusal to
// spawn; none of these are recoverable at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttestError {
    // No attestation trailer present on the capsule.
    Missing,
    // Trailer bytes are malformed or truncated.
    Malformed,
    // The Groth16 proof failed cryptographic verification.
    ProofInvalid,
    // The commitment public input does not match the recomputed commitment.
    CommitmentMismatch,
    // The capsule hash in the proof does not match the real capsule bytes.
    HashMismatch,
    // The capability mask in the proof does not match the granted capabilities.
    CapabilityMismatch,
}

impl AttestError {
    pub fn as_str(&self) -> &'static str {
        match self {
            AttestError::Missing => "capsule attestation trailer missing",
            AttestError::Malformed => "capsule attestation trailer malformed",
            AttestError::ProofInvalid => "capsule attestation proof invalid",
            AttestError::CommitmentMismatch => "capsule attestation commitment mismatch",
            AttestError::HashMismatch => "capsule attestation hash does not bind to bytes",
            AttestError::CapabilityMismatch => "capsule attestation caps do not bind to grant",
        }
    }
}
