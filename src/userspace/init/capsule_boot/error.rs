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

extern crate alloc;

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

pub(super) fn message(prefix: &str, err: SpawnError) -> alloc::string::String {
    match err {
        SpawnError::FeatureDisabled => {
            alloc::format!("{}: capsule binary not embedded (feature off)", prefix)
        }
        SpawnError::ElfLoad => alloc::format!("{}: capsule ELF load failed", prefix),
        SpawnError::ProcessCreation => alloc::format!("{}: process creation failed", prefix),
        SpawnError::AddressSpace => {
            alloc::format!("{}: address space allocation failed", prefix)
        }
        SpawnError::EndpointCollision => {
            alloc::format!("{}: service endpoint registration failed", prefix)
        }
        SpawnError::NonosIdCertRejected(reason) => super::cert_reason::message(prefix, reason),
        SpawnError::ManifestRejected(reason) => super::manifest_reason::message(prefix, reason),
        SpawnError::AttestationRejected => {
            alloc::format!("{}: capsule ZK attestation failed", prefix)
        }
    }
}
