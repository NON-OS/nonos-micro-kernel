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
    TEXT_EDITOR_ATTESTATION_BYTES, TEXT_EDITOR_ELF, TEXT_EDITOR_MANIFEST_BYTES,
    TEXT_EDITOR_NONOS_ID_CERT_BYTES,
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

const SERVICE_NAME: &str = "app.text_editor";
const SERVICE_PORT: u32 = 4726;
const REPLY_INBOX: &str = "endpoint.app.text_editor.reply";
const REPLY_PORT: u32 = 4727;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

// Extra window endpoints, each declared in the signed manifest. Ordered, so the
// lowest-numbered free one is taken.
const TEXT_EDITOR_INSTANCES: &[InstanceEndpoint] = &[
    InstanceEndpoint {
        name: "app.text_editor.1",
        port: 4830,
        reply_inbox: "endpoint.app.text_editor.1.reply",
        reply_port: 4831,
    },
    InstanceEndpoint {
        name: "app.text_editor.2",
        port: 4832,
        reply_inbox: "endpoint.app.text_editor.2.reply",
        reply_port: 4833,
    },
];

// Spawn the next free text editor window on demand. Same signed artifacts as
// boot, a fresh pid, and thus its own compositor window.
pub fn spawn_text_editor_instance() -> Result<u32, SpawnError> {
    spawn_next_instance(&InstanceSpawn {
        elf: TEXT_EDITOR_ELF,
        cert: TEXT_EDITOR_NONOS_ID_CERT_BYTES,
        manifest: TEXT_EDITOR_MANIFEST_BYTES,
        attestation: TEXT_EDITOR_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::CoreExec.bit()
            | Capability::IPC.bit()
            | Capability::Memory.bit()
            | Capability::FileSystem.bit()
            | Capability::GraphicsDisplayQuery.bit()
            | Capability::GraphicsSurfaceCreate.bit(),
        instances: TEXT_EDITOR_INSTANCES,
        debug_tag: b"[TEXT_EDITOR-INSTANCE] elf error:",
    })
}

pub fn spawn_text_editor_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: TEXT_EDITOR_ELF,
        nonos_id_cert_bytes: TEXT_EDITOR_NONOS_ID_CERT_BYTES,
        manifest_bytes: TEXT_EDITOR_MANIFEST_BYTES,
        attestation_trailer: TEXT_EDITOR_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::CoreExec.bit()
            | Capability::IPC.bit()
            | Capability::Memory.bit()
            | Capability::FileSystem.bit()
            | Capability::GraphicsDisplayQuery.bit()
            | Capability::GraphicsSurfaceCreate.bit(),
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
