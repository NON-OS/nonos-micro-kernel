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

//! Spawn an extra instance of an already-embedded, attested app capsule so it
//! gets its own compositor window. Every instance reuses the same signed ELF,
//! certificate, manifest and attestation as the boot instance; only the service
//! and reply endpoint differ, and each such endpoint is declared in the signed
//! manifest, so nothing here bypasses signing. The endpoint set is a static
//! table matching the manifest's instance endpoints; the verified spawn path
//! still re-checks that each is declared (endpoint drift) before registering it.

use super::{spawn_verified, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};
use crate::services::registry::{lookup_port, lookup_service};

/// One extra window's service + reply endpoint, all declared in the manifest.
pub struct InstanceEndpoint {
    pub name: &'static str,
    pub port: u32,
    pub reply_inbox: &'static str,
    pub reply_port: u32,
}

/// The embedded artifacts of one app capsule plus its instance endpoint table.
pub struct InstanceSpawn {
    pub elf: &'static [u8],
    pub cert: &'static [u8],
    pub manifest: &'static [u8],
    pub attestation: &'static [u8],
    pub target_triple: &'static str,
    pub requested_caps: u64,
    pub instances: &'static [InstanceEndpoint],
    pub debug_tag: &'static [u8],
}

extern crate alloc;

/// Spawn the next free instance window and return its pid. Each instance is a
/// fresh, fully attested capsule; closing its window exits it, which zeroizes
/// its RAM and frees the slot, so the next spawn here reuses that slot with a
/// brand new capsule rather than reviving a lingering one. Every declared slot
/// being live means the on-screen cap is reached; the caller then focuses an
/// existing window instead. The instance is marked ephemeral through its argv
/// so the app skeleton exits on close rather than idling.
pub fn spawn_next(app: &InstanceSpawn) -> Result<u32, SpawnError> {
    let slot = app
        .instances
        .iter()
        .find(|e| lookup_service(e.name).is_none() && lookup_port(e.port).is_none())
        .ok_or(SpawnError::EndpointCollision)?;

    let trust = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: slot.name,
        service_port: slot.port,
        reply_inbox: slot.reply_inbox,
        reply_port: slot.reply_port,
        elf: app.elf,
        nonos_id_cert_bytes: app.cert,
        manifest_bytes: app.manifest,
        attestation_trailer: app.attestation,
        target_triple: app.target_triple,
        requested_caps: app.requested_caps,
        debug_tag: app.debug_tag,
    };
    // Match the boot spawn path exactly: it passes None here, so the
    // certificate temporal window is treated the same for an on-demand
    // instance as for the boot instance.
    let pid = spawn_verified(&spec, &trust, None)?;
    // Tag the instance ephemeral. The app skeleton reads this and exits (full
    // teardown, RAM zeroized) when its window closes, instead of lingering
    // idle, so a closed tab leaves no resident capsule and a relaunch is a
    // fresh spawn. Set before the app is scheduled far enough to read argv.
    crate::process::with_process(pid, |pcb| {
        *pcb.argv.lock() = alloc::vec![alloc::string::String::from(INSTANCE_ARG)];
    });
    Ok(pid)
}

/// The argv marker that tells the app skeleton it is an on-demand window
/// instance and must exit when its window closes.
pub const INSTANCE_ARG: &str = "--nonos-window-instance";
