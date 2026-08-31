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

use alloc::vec::Vec;

const TPM_ALG_ECC: u16 = 0x0023;
const TPM_ALG_SHA256: u16 = 0x000B;
const TPM_ALG_NULL: u16 = 0x0010;
const TPM_ALG_ECDSA: u16 = 0x0018;
const TPM_ECC_NIST_P256: u16 = 0x0003;

/// `fixedTPM | fixedParent | sensitiveDataOrigin | userWithAuth | restricted
/// | sign`.
///
/// `restricted` is the one that matters for attestation. A restricted signing
/// key will only sign digests the TPM itself produced, so it cannot be used to
/// sign an attestation structure handed to it from outside. Without it, a
/// quote proves nothing: anyone able to talk to the TPM could have it sign a
/// fabricated `TPMS_ATTEST`.
const OBJECT_ATTRIBUTES: u32 = 0x0005_0072;

/// The template whose derivation gives this machine its identity.
///
/// A primary key is derived from the hierarchy's seed and this template, so
/// the same TPM produces the same key every boot without anything being
/// stored. That is what lets an amnesic system still have an identity a
/// counterparty can pin across reboots: change one byte here and the machine
/// becomes a different one to every verifier.
pub(super) fn ak_template() -> Vec<u8> {
    let mut pubarea = Vec::with_capacity(64);
    pubarea.extend_from_slice(&TPM_ALG_ECC.to_be_bytes());
    pubarea.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    pubarea.extend_from_slice(&OBJECT_ATTRIBUTES.to_be_bytes());
    // authPolicy: empty, so authorization is the empty password rather than a
    // policy session.
    pubarea.extend_from_slice(&0u16.to_be_bytes());

    // TPMS_ECC_PARMS: no symmetric algorithm on a signing key, ECDSA over
    // SHA-256, P-256, and no key derivation function.
    pubarea.extend_from_slice(&TPM_ALG_NULL.to_be_bytes());
    pubarea.extend_from_slice(&TPM_ALG_ECDSA.to_be_bytes());
    pubarea.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    pubarea.extend_from_slice(&TPM_ECC_NIST_P256.to_be_bytes());
    pubarea.extend_from_slice(&TPM_ALG_NULL.to_be_bytes());

    // unique: both coordinates empty, so the TPM fills them from the
    // derivation rather than from anything supplied here.
    pubarea.extend_from_slice(&0u16.to_be_bytes());
    pubarea.extend_from_slice(&0u16.to_be_bytes());

    let mut out = Vec::with_capacity(2 + pubarea.len());
    out.extend_from_slice(&(pubarea.len() as u16).to_be_bytes());
    out.extend_from_slice(&pubarea);
    out
}
