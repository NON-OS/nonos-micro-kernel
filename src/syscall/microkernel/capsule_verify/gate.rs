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

use crate::kernel_core::process_spawn::capsule_spawn::Tier;
use crate::security::capsule_attest::verify_capsule_attestation;
use crate::security::capsule_manifest::CapsuleManifest;
use crate::syscall::microkernel::errnos::ERRNO_ACCES;

pub(super) fn check(
    tier: &Tier,
    manifest: &CapsuleManifest,
    elf: &[u8],
    trailer: &[u8],
) -> Result<(), i64> {
    if trailer.is_empty() {
        return match tier {
            Tier::Publisher => Ok(()),
            Tier::Enrolled => rollout_verdict(),
        };
    }
    match verify_capsule_attestation(trailer, elf, manifest.required_caps) {
        Ok(()) => Ok(()),
        Err(_) => rollout_verdict(),
    }
}

fn rollout_verdict() -> Result<(), i64> {
    if cfg!(feature = "nonos-zk-rollout") {
        Ok(())
    } else {
        Err(ERRNO_ACCES)
    }
}
