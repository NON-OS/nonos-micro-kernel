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

#[cfg(not(feature = "nonos-production"))]
pub struct CapsuleSpec {
    pub name: &'static str,
    pub service_port: u32,
    pub reply_inbox: &'static str,
    pub reply_port: u32,
    pub elf: &'static [u8],
    pub caps_bits: u64,
    pub debug_tag: &'static [u8],
}

pub struct CapsuleSpecVerified {
    pub name: &'static str,
    pub service_port: u32,
    pub reply_inbox: &'static str,
    pub reply_port: u32,
    pub elf: &'static [u8],
    pub nonos_id_cert_bytes: &'static [u8],
    pub manifest_bytes: &'static [u8],
    pub target_triple: &'static str,
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
