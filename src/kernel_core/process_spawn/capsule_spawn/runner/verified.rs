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

use crate::security::dev_roots::Authority;
use crate::security::nonos_trust_anchor::NonosTrustAnchorPolicy;

use super::super::attested_parent::AttestedParent;
use super::super::spec::{CapsuleSpecVerified, SpawnError};
use super::install::{install, InstallParams};
use super::preflight;

// Caps installed on the PCB come from the verified manifest, never
// from spec.requested_caps. requested_caps is only the upper bound
// the spawn site is willing to grant for optional caps.
pub fn spawn_verified(
    spec: &CapsuleSpecVerified,
    trust_anchor: &NonosTrustAnchorPolicy,
    now_ms: Option<u64>,
) -> Result<u32, SpawnError> {
    spawn_verified_as(spec, trust_anchor, now_ms, None)
}

// Same as `spawn_verified`, but attributes the spawned process to
// `on_behalf_of` (a kernel-attested pid) instead of the caller. Used by the
// capsule-load-from-store path when the caller holds spawn-broker authority.
pub(crate) fn spawn_verified_as(
    spec: &CapsuleSpecVerified,
    trust_anchor: &NonosTrustAnchorPolicy,
    now_ms: Option<u64>,
    on_behalf_of: Option<AttestedParent>,
) -> Result<u32, SpawnError> {
    crate::sys::bench::mark_named(b"capsule_spawn_start", spec.name.as_bytes());
    let preflighted = match preflight::run(spec, trust_anchor, now_ms) {
        Ok(preflighted) => preflighted,
        Err(err) => {
            crate::sys::bench::mark_named(b"capsule_verify_fail", spec.name.as_bytes());
            return Err(err);
        }
    };
    crate::sys::bench::mark_named(b"capsule_preflight_ok", spec.name.as_bytes());
    let pid = install(&InstallParams {
        name: spec.name,
        service_port: spec.service_port,
        reply_inbox: spec.reply_inbox,
        reply_port: spec.reply_port,
        elf: spec.elf,
        caps_bits: preflighted.install_caps,
        debug_tag: spec.debug_tag,
        on_behalf_of,
    })?;
    /*
     * First point at which a pid exists. What goes in is the measurement the
     * gate checked, never one recomputed from the image. A capsule that ran
     * on a publisher signature alone is recorded under that authority rather
     * than left out: the registry claims to be complete, and a receipt that
     * omits a running program is the one lie a verifier cannot catch.
     */
    let (measurement, authority) = match preflighted.proved {
        Some(proved) => (proved.measurement, proved.authority),
        None => (preflighted.capsule_id, Authority::Publisher),
    };
    crate::security::attest_registry::record_attested(
        pid,
        measurement,
        preflighted.install_caps,
        authority,
    );
    Ok(pid)
}
