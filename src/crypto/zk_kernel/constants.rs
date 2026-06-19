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

//! Domain separation constants for zero-knowledge proofs.

/// Domain separator for Schnorr proofs.
pub const DOM_SCHNORR: &[u8] = b"NONOS_ZK_SCHNORR_V1";

/// Domain separator for Pedersen commitments.
pub const DOM_PEDERSEN: &[u8] = b"NONOS_ZK_PEDERSEN_V1";

/// Domain separator for range proofs.
pub const DOM_RANGE: &[u8] = b"NONOS_ZK_RANGE_V1";

/// Domain separator for equality proofs.
pub const DOM_EQUALITY: &[u8] = b"NONOS_ZK_EQUALITY_V1";

/// Domain separator for Merkle membership proofs.
pub const DOM_MERKLE: &[u8] = b"NONOS_ZK_MERKLE_V1";

/// Domain separator for Sigma protocols.
pub const DOM_SIGMA: &[u8] = b"NONOS_ZK_SIGMA_V1";

/// Domain separator for PLONK proofs.
pub const DOM_PLONK: &[u8] = b"NONOS_ZK_PLONK_V1";

/// Domain separator for enrolled-secret attestation challenges.
pub(crate) const DOM_ATTEST: &[u8] = b"NONOS_ZK_ATTEST_V1";

/// Domain separator for attestation Merkle leaves.
pub(crate) const DOM_MERKLE_LEAF: &[u8] = b"NONOS_ZK_MERKLE_LEAF_V1";

/// The L scalar field modulus (curve25519 group order).
pub const L: [u8; 32] = [
    0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x1e, 0x8d, 0xce, 0x4c, 0xcd, 0x65, 0xa0, 0x2f, 0x8a, 0x01,
    0x4f, 0xd9, 0x12, 0x6b, 0x83, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
];
