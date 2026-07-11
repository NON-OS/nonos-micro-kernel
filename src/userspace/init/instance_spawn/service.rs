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

use super::queue::{self, PendingApp};
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

/// Perform every queued window-instance spawn. Called from `init_loop`, so
/// this runs in init's context, the same environment the boot spawns use.
/// Each spawn still goes through the full verified, attested pipeline; a
/// rejection (every declared slot already live, or a load error) is logged
/// with its cause and skipped so one bad request cannot wedge the drain.
pub(crate) fn service() {
    for app in queue::take() {
        let result = match app {
            PendingApp::Terminal => spawn_terminal(),
            PendingApp::Browser => spawn_browser(),
        };
        match result {
            // Deliver the focus frame the app skeleton waits for, so the
            // new instance builds its window instead of sitting idle.
            Ok(pid) => super::boot_frame::boot(pid),
            Err(e) => {
                crate::sys::serial::print(b"[SPAWN-INSTANCE] deferred spawn rejected err=");
                crate::sys::serial::println(spawn_error_name(e));
            }
        }
    }
}

fn spawn_error_name(e: SpawnError) -> &'static [u8] {
    match e {
        SpawnError::FeatureDisabled => b"FeatureDisabled",
        SpawnError::ElfLoad => b"ElfLoad",
        SpawnError::ProcessCreation => b"ProcessCreation",
        SpawnError::AddressSpace => b"AddressSpace",
        SpawnError::EndpointCollision => b"EndpointCollision",
        SpawnError::NonosIdCertRejected(_) => b"CertRejected",
        SpawnError::ManifestRejected(_) => b"ManifestRejected",
        SpawnError::AttestationRejected => b"AttestationRejected",
    }
}

#[cfg(feature = "nonos-capsule-terminal")]
fn spawn_terminal() -> Result<u32, SpawnError> {
    crate::userspace::capsule_terminal::spawn_terminal_instance()
}

#[cfg(not(feature = "nonos-capsule-terminal"))]
fn spawn_terminal() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}

#[cfg(feature = "nonos-capsule-browser")]
fn spawn_browser() -> Result<u32, SpawnError> {
    crate::userspace::capsule_browser::spawn_browser_instance()
}

#[cfg(not(feature = "nonos-capsule-browser"))]
fn spawn_browser() -> Result<u32, SpawnError> {
    Err(SpawnError::FeatureDisabled)
}
