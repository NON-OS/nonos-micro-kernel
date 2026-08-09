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
    TERMINAL_ATTESTATION_BYTES, TERMINAL_ELF, TERMINAL_MANIFEST_BYTES, TERMINAL_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{
    self, spawn_next_instance, CapsuleSpecVerified, InstanceEndpoint, InstanceSpawn,
};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

const SERVICE_NAME: &str = "app.terminal";
const SERVICE_PORT: u32 = 4722;
const REPLY_INBOX: &str = "endpoint.app.terminal.reply";
const REPLY_PORT: u32 = 4723;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

// The requested-caps ceiling, shared by the boot instance and every extra
// window so the attestation context (which binds granted caps) is identical.
fn terminal_caps() -> u64 {
    Capability::CoreExec.bit()
        | Capability::IPC.bit()
        | Capability::Network.bit()
        | Capability::Memory.bit()
        | Capability::Crypto.bit()
        | Capability::FileSystem.bit()
        | Capability::GraphicsDisplayQuery.bit()
        | Capability::GraphicsSurfaceCreate.bit()
}

// Extra window endpoints, each declared in the signed manifest (see the
// terminal Capsule.mk). Ordered, so the lowest-numbered free one is taken.
const TERMINAL_INSTANCES: &[InstanceEndpoint] = &[
    InstanceEndpoint {
        name: "app.terminal.1",
        port: 4740,
        reply_inbox: "endpoint.app.terminal.1.reply",
        reply_port: 4741,
    },
    InstanceEndpoint {
        name: "app.terminal.2",
        port: 4742,
        reply_inbox: "endpoint.app.terminal.2.reply",
        reply_port: 4743,
    },
    InstanceEndpoint {
        name: "app.terminal.3",
        port: 4744,
        reply_inbox: "endpoint.app.terminal.3.reply",
        reply_port: 4745,
    },
];

// Spawn the next free terminal window on demand. Same signed artifacts as boot,
// a fresh pid, and thus its own compositor window.
pub fn spawn_terminal_instance() -> Result<u32, SpawnError> {
    spawn_next_instance(&InstanceSpawn {
        elf: TERMINAL_ELF,
        cert: TERMINAL_NONOS_ID_CERT_BYTES,
        manifest: TERMINAL_MANIFEST_BYTES,
        attestation: TERMINAL_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: terminal_caps(),
        instances: TERMINAL_INSTANCES,
        debug_tag: b"[TERMINAL-INSTANCE] elf error:",
    })
}

pub fn spawn_terminal_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: TERMINAL_ELF,
        nonos_id_cert_bytes: TERMINAL_NONOS_ID_CERT_BYTES,
        manifest_bytes: TERMINAL_MANIFEST_BYTES,
        attestation_trailer: TERMINAL_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: terminal_caps(),
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
