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

use crate::kernel_core::process_spawn::capsule_spawn::{LoadError, SpawnError};
use crate::syscall::microkernel::errnos::{ERRNO_ACCES, ERRNO_EXIST, ERRNO_INVAL};

// Map a loader failure to the negative errno the syscall returns. A malformed
// manifest is an invalid argument; a rejected signature, manifest, or
// attestation, and a trust-anchor decode failure, surface as access denied. A
// collision on the service or reply endpoint means a live instance already
// holds the name, which is a conflict rather than a verification failure and
// must not be reported as one.
pub(super) fn load_errno(e: LoadError) -> i64 {
    match e {
        LoadError::Manifest => ERRNO_INVAL,
        LoadError::Spawn(SpawnError::EndpointCollision) => ERRNO_EXIST,
        LoadError::TrustAnchor | LoadError::Spawn(_) => ERRNO_ACCES,
    }
}
