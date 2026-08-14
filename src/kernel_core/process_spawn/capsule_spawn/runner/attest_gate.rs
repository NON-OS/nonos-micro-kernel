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
use crate::security::capsule_attest::{verify_capsule_attestation, Proved};

/// Returns what was proved and who proved it, so the spawn path can record
/// both once a pid exists. `None` only on the rollout path, where nothing was
/// proved at all.
pub(crate) fn attest_gate(
    spec: &CapsuleSpecVerified,
    install_caps: u64,
) -> Result<Option<Proved>, SpawnError> {
    let trailer = spec.attestation_trailer;
    if trailer.is_empty() {
        crate::sys::bench::mark_named(b"capsule_attest_none", spec.name.as_bytes());
        crate::sys::serial::print(b"[ZK-ATTEST] none ");
        crate::sys::serial::print(spec.name.as_bytes());
        crate::sys::serial::print(b"\n");
        #[cfg(not(feature = "nonos-zk-rollout"))]
        return Err(SpawnError::AttestationRejected);
        #[cfg(feature = "nonos-zk-rollout")]
        return Ok(None);
    }
    match verify_capsule_attestation(trailer, spec.elf, install_caps) {
        Ok(proved) => {
            crate::sys::bench::mark_named(b"capsule_attest_ok", spec.name.as_bytes());
            crate::sys::serial::print(b"[ZK-ATTEST] ok ");
            crate::sys::serial::print(spec.name.as_bytes());
            // The authority is on the line: a reader of the boot log should be
            // able to tell shipped capsules from ones built on this machine.
            crate::sys::serial::print(b" ");
            crate::sys::serial::print(proved.authority.as_str().as_bytes());
            crate::sys::serial::print(b"\n");
            Ok(Some(proved))
        }
        Err(e) => {
            crate::sys::bench::mark_named(b"capsule_attest_fail", spec.name.as_bytes());
            crate::sys::serial::print(b"[ZK-ATTEST] FAIL ");
            crate::sys::serial::print(spec.name.as_bytes());
            crate::sys::serial::print(b": ");
            crate::sys::serial::print(e.as_str().as_bytes());
            crate::sys::serial::print(b"\n");
            #[cfg(not(feature = "nonos-zk-rollout"))]
            {
                return Err(SpawnError::AttestationRejected);
            }
            #[cfg(feature = "nonos-zk-rollout")]
            Ok(None)
        }
    }
}
