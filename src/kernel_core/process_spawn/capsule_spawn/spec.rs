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

use crate::security::capsule_manifest::ManifestVerifyError;
use crate::security::nonos_id_cert::IdCertVerifyError;

#[cfg(feature = "nonos-dev-unverified-capsules")]
pub struct CapsuleSpec {
    pub name: &'static str,
    pub service_port: u32,
    pub reply_inbox: &'static str,
    pub reply_port: u32,
    pub elf: &'static [u8],
    pub caps_bits: u64,
    pub debug_tag: &'static [u8],
}

/*
 * Borrowed for the length of the spawn call, not for the life of the machine.
 *
 * A baked capsule's artifacts come from `include_bytes!` and are 'static, and
 * they coerce into this without anyone noticing. A capsule loaded at runtime
 * owns its four blobs in a Vec that the caller drops when the call returns.
 *
 * The lifetime used to be 'static for both, which meant the runtime path had
 * to leak: `Box::leak` on every artifact, on the success path and on the
 * failure path alike. Nothing needs them to live that long. The endpoint
 * registry copies the name into a String, the process control block copies it
 * again, and the ELF is mapped into the process address space before the call
 * returns. After that the bytes have no reader.
 */
pub struct CapsuleSpecVerified<'a> {
    pub name: &'a str,
    pub service_port: u32,
    pub reply_inbox: &'a str,
    pub reply_port: u32,
    pub elf: &'a [u8],
    pub nonos_id_cert_bytes: &'a [u8],
    pub manifest_bytes: &'a [u8],
    /*
     * Per-capsule ZK attestation trailer (NZKCAPS1), embedded alongside the
     * manifest. Empty when the capsule has no sidecar in this build.
     */
    pub attestation_trailer: &'a [u8],
    pub target_triple: &'a str,
    pub requested_caps: u64,
    pub debug_tag: &'static [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnError {
    FeatureDisabled,
    ElfLoad,
    ProcessCreation,
    AddressSpace,
    EndpointCollision,
    /// The reply inbox name is empty or longer than a process stores.
    InboxName,
    NonosIdCertRejected(IdCertVerifyError),
    ManifestRejected(ManifestVerifyError),
    AttestationRejected,
}

impl From<IdCertVerifyError> for SpawnError {
    fn from(e: IdCertVerifyError) -> Self {
        SpawnError::NonosIdCertRejected(e)
    }
}

impl From<ManifestVerifyError> for SpawnError {
    fn from(e: ManifestVerifyError) -> Self {
        SpawnError::ManifestRejected(e)
    }
}
