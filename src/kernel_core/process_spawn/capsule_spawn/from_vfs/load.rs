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

use super::artifacts::CapsuleArtifacts;
use super::error::LoadError;
use super::leak::{leak_bytes, leak_str};
use crate::kernel_core::process_spawn::capsule_spawn::{spawn_verified, CapsuleSpecVerified};
use crate::security::capsule_manifest::{decode as decode_manifest, CapsuleManifest, EndpointKind};
use crate::security::nonos_trust_anchor::{decode as decode_trust, BAKED_TRUST_ANCHOR_POLICY};

// Spawn a capsule whose four artifacts were read from the store by the
// installer. The service name, endpoints, and target triple are taken from the
// capsule's own signed manifest, so a loaded capsule registers exactly what it
// declares and the caller cannot misname or misroute it. The artifacts then go
// through the same verified spawn path baked capsules use, with every identity,
// manifest, publisher signature, capability, and attestation check intact.
// requested_caps is the upper bound for optional caps, identical to a baked
// spawn site; the verified manifest still decides what is actually granted.
pub fn load_capsule_from_vfs(
    artifacts: CapsuleArtifacts,
    requested_caps: u64,
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
        requested_caps,
        debug_tag: b"[RUNTIME-LOAD] elf error:",
    };
    let trust = decode_trust(BAKED_TRUST_ANCHOR_POLICY).map_err(|_| LoadError::TrustAnchor)?;
    // Feed the real clock so the certificate validity window is enforced; a zero
    // clock (not yet live) falls back to the baked behavior of skipping the
    // temporal check rather than rejecting every certificate.
    let now = crate::time::timestamp_millis();
    let now_ms = if now == 0 { None } else { Some(now) };
    spawn_verified(&spec, &trust, now_ms).map_err(LoadError::Spawn)
}

fn endpoint(m: &CapsuleManifest, kind: EndpointKind) -> Result<(&str, u32), LoadError> {
    m.endpoints
        .iter()
        .find(|e| e.kind == kind)
        .map(|e| (e.name_str(), e.port))
        .ok_or(LoadError::Manifest)
}
