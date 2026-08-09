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

use super::super::spec::{CapsuleSpecVerified, SpawnError};

pub(crate) fn publisher_gate(
    spec: &CapsuleSpecVerified,
    namespace: &str,
    attest_caps: u64,
) -> Result<(), SpawnError> {
    if !matches!(super::tier::classify(namespace), super::tier::Tier::Publisher) {
        return Err(SpawnError::AttestationRejected);
    }
    if spec.attestation_trailer.is_empty() {
        crate::sys::bench::mark_named(b"capsule_attest_pub", spec.name.as_bytes());
        crate::sys::serial::print(b"[ZK-ATTEST] pub ");
        crate::sys::serial::print(spec.name.as_bytes());
        crate::sys::serial::print(b"\n");
        return Ok(());
    }
    super::attest_gate::attest_gate(spec, attest_caps)
}
