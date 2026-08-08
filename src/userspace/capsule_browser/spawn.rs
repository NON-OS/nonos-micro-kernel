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

use super::embed::{
    BROWSER_ATTESTATION_BYTES, BROWSER_ELF, BROWSER_MANIFEST_BYTES, BROWSER_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
use crate::kernel_core::process_spawn::capsule_spawn::{
    self, spawn_next_instance, CapsuleSpecVerified, InstanceEndpoint, InstanceSpawn,
};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "app.browser";
const SERVICE_PORT: u32 = 4760;
const REPLY_INBOX: &str = "endpoint.app.browser.reply";
const REPLY_PORT: u32 = 4761;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

// Shared caps ceiling for the boot instance and every extra window, so the
// attestation context (which binds granted caps) matches across instances.
fn browser_caps() -> u64 {
    Capability::CoreExec.bit()
        | Capability::IPC.bit()
        | Capability::Network.bit()
        | Capability::Memory.bit()
        | Capability::Crypto.bit()
        | Capability::GraphicsDisplayQuery.bit()
        | Capability::GraphicsSurfaceCreate.bit()
}

// Extra window endpoints, each declared in the signed manifest (browser
// Capsule.mk). Ordered, so the lowest-numbered free one is taken.
const BROWSER_INSTANCES: &[InstanceEndpoint] = &[
    InstanceEndpoint {
        name: "app.browser.1",
        port: 4762,
        reply_inbox: "endpoint.app.browser.1.reply",
        reply_port: 4763,
    },
    InstanceEndpoint {
        name: "app.browser.2",
        port: 4764,
        reply_inbox: "endpoint.app.browser.2.reply",
        reply_port: 4765,
    },
    InstanceEndpoint {
        name: "app.browser.3",
        port: 4766,
        reply_inbox: "endpoint.app.browser.3.reply",
        reply_port: 4767,
    },
];

pub fn spawn_browser_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: BROWSER_ELF,
        nonos_id_cert_bytes: BROWSER_NONOS_ID_CERT_BYTES,
        manifest_bytes: BROWSER_MANIFEST_BYTES,
        attestation_trailer: BROWSER_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: browser_caps(),
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}

// Spawn the next free browser window on demand: same signed artifacts as boot,
// a fresh pid, its own compositor window.
pub fn spawn_browser_instance() -> Result<u32, SpawnError> {
    spawn_next_instance(&InstanceSpawn {
        elf: BROWSER_ELF,
        cert: BROWSER_NONOS_ID_CERT_BYTES,
        manifest: BROWSER_MANIFEST_BYTES,
        attestation: BROWSER_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: browser_caps(),
        instances: BROWSER_INSTANCES,
        debug_tag: b"[BROWSER-INSTANCE] elf error:",
    })
}
