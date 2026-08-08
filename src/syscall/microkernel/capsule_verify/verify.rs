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

//! HAND-SYNCED MIRROR of the verification chain in
//! `src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs`. This is a
//! second copy of that chain, not a call into it: `preflight::run` sits on the
//! spawn path, whose `.text` layout and allocation order are load-bearing for
//! several intermittent faults, and MkCapsuleVerify must be answerable without
//! perturbing it. The two were verified equivalent when this was written and
//! must be changed together — a rule tightened in `preflight.rs` alone leaves
//! this syscall admitting artifacts the spawner would later reject, and a rule
//! tightened here alone rejects artifacts that spawn fine.

use crate::kernel_core::process_spawn::capsule_spawn::{classify_tier, validity_now_ms, Tier};
use crate::security::capsule_manifest::{
    decode as decode_manifest, verify_with_publisher, CapsuleManifest, DeclaredEndpoint,
    EndpointKind,
};
use crate::security::nonos_id_cert::{
    decode as decode_id_cert, verify as verify_id_cert, NONOS_PRODUCTION_POLICY,
};
use crate::security::nonos_trust_anchor::{decode as decode_trust, BAKED_TRUST_ANCHOR_POLICY};
use crate::syscall::microkernel::errnos::{ERRNO_ACCES, ERRNO_INVAL};

pub(super) struct Verified {
    pub manifest: CapsuleManifest,
    pub tier: Tier,
    pub install_caps: u64,
}

pub(super) fn run(
    elf: &[u8],
    cert_bytes: &[u8],
    manifest_bytes: &[u8],
    trailer: &[u8],
) -> Result<Verified, i64> {
    let manifest = decode_manifest(manifest_bytes).map_err(|_| ERRNO_INVAL)?;
    let service = endpoint(&manifest, EndpointKind::Service).ok_or(ERRNO_INVAL)?;
    let reply = endpoint(&manifest, EndpointKind::Reply).ok_or(ERRNO_INVAL)?;
    let trust = decode_trust(BAKED_TRUST_ANCHOR_POLICY).map_err(|_| ERRNO_ACCES)?;
    let now_ms = validity_now_ms(crate::sys::unix_ms());
    let cert = decode_id_cert(cert_bytes).map_err(|_| ERRNO_ACCES)?;
    let verified_id = verify_id_cert(cert_bytes, &trust, &NONOS_PRODUCTION_POLICY, now_ms)
        .map_err(|_| ERRNO_ACCES)?;
    let declared = [
        DeclaredEndpoint { kind: EndpointKind::Service, port: service.1, name: service.0 },
        DeclaredEndpoint { kind: EndpointKind::Reply, port: reply.1, name: reply.0 },
    ];
    let granted = manifest.required_caps | manifest.optional_caps;
    let verification = verify_with_publisher(
        manifest_bytes,
        cert_bytes,
        &cert,
        &verified_id,
        &trust,
        &NONOS_PRODUCTION_POLICY,
        elf,
        manifest.target_triple_str(),
        granted,
        &declared,
    )
    .map_err(|_| ERRNO_ACCES)?;
    let tier = classify_tier(manifest.namespace_str());
    super::gate::check(&tier, &manifest, elf, trailer)?;
    Ok(Verified { manifest, tier, install_caps: verification.1 })
}

fn endpoint(m: &CapsuleManifest, kind: EndpointKind) -> Option<(&str, u32)> {
    m.endpoints.iter().find(|e| e.kind == kind).map(|e| (e.name_str(), e.port))
}
