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

use super::super::artifacts::CapsuleArtifacts;
use super::super::error::LoadError;
use super::super::leak::{leak_bytes, leak_str};
use super::endpoint::endpoint;
use crate::kernel_core::process_spawn::capsule_spawn::{
    spawn_verified_as, AttestedParent, CapsuleSpecVerified,
};
use crate::security::capsule_manifest::{decode as decode_manifest, EndpointKind};
use crate::security::nonos_trust_anchor::{decode as decode_trust, BAKED_TRUST_ANCHOR_POLICY};

// Spawn a capsule whose four artifacts were read from the store by the
// installer. The service name, endpoints, and target triple are taken from the
// capsule's own signed manifest, so a loaded capsule registers exactly what it
// declares and the caller cannot misname or misroute it. The artifacts then go
// through the same verified spawn path baked capsules use, with every identity,
// manifest, publisher signature, capability, and attestation check intact.
// requested_caps is the upper bound for optional caps, identical to a baked
// spawn site; the verified manifest still decides what is actually granted.
pub(crate) fn load_capsule_from_vfs(
    artifacts: CapsuleArtifacts,
    requested_caps: u64,
    args: &[u8],
    on_behalf_of: Option<AttestedParent>,
) -> Result<u32, LoadError> {
    let manifest = decode_manifest(&artifacts.manifest).map_err(|_| LoadError::Manifest)?;
    let (service_name, service_port) = endpoint(&manifest, EndpointKind::Service)?;
    let (reply_name, reply_port) = endpoint(&manifest, EndpointKind::Reply)?;

    let spec = CapsuleSpecVerified {
        name: leak_str(service_name),
        service_port,
        reply_inbox: leak_str(reply_name),
        reply_port,
        elf: leak_bytes(artifacts.elf),
        nonos_id_cert_bytes: leak_bytes(artifacts.cert),
        manifest_bytes: leak_bytes(artifacts.manifest),
        attestation_trailer: leak_bytes(artifacts.trailer),
        target_triple: leak_str(manifest.target_triple_str()),
        requested_caps: requested_caps & (manifest.required_caps | manifest.optional_caps),
        debug_tag: b"[RUNTIME-LOAD] elf error:",
    };
    let trust = decode_trust(BAKED_TRUST_ANCHOR_POLICY).map_err(|_| LoadError::TrustAnchor)?;
    // Enforce the certificate validity window against the wall clock, not
    // uptime-since-boot: a real `valid_from_ms` is a wall-clock epoch, so
    // comparing it to uptime rejected every runtime-loaded capsule as
    // NotYetValid. Before the clock is set the gate returns None and the
    // signature and trust anchor still gate the load.
    let now_ms = super::super::validity_clock::validity_now_ms(crate::sys::unix_ms());
    let pid = spawn_verified_as(&spec, &trust, now_ms, on_behalf_of).map_err(LoadError::Spawn)?;
    if !args.is_empty() {
        let argv: alloc::vec::Vec<alloc::string::String> = args
            .split(|&b| b == 0)
            .filter(|s| !s.is_empty())
            .map(|s| alloc::string::String::from_utf8_lossy(s).into_owned())
            .collect();
        crate::process::with_process(pid, |pcb| *pcb.argv.lock() = argv);
    }
    Ok(pid)
}
