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
    SNAKE_ATTESTATION_BYTES, SNAKE_ELF, SNAKE_MANIFEST_BYTES, SNAKE_NONOS_ID_CERT_BYTES,
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

const SERVICE_NAME: &str = "app.snake";
const SERVICE_PORT: u32 = 4732;
const REPLY_INBOX: &str = "endpoint.app.snake.reply";
const REPLY_PORT: u32 = 4733;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

// Extra window endpoints, each declared in the signed manifest. Ordered, so the
// lowest-numbered free one is taken.
const SNAKE_INSTANCES: &[InstanceEndpoint] = &[
    InstanceEndpoint {
        name: "app.snake.1",
        port: 4850,
        reply_inbox: "endpoint.app.snake.1.reply",
        reply_port: 4851,
    },
    InstanceEndpoint {
        name: "app.snake.2",
        port: 4852,
        reply_inbox: "endpoint.app.snake.2.reply",
        reply_port: 4853,
    },
];

// Spawn the next free snake window on demand. Same signed artifacts as boot, a
// fresh pid, and thus its own compositor window.
pub fn spawn_snake_instance() -> Result<u32, SpawnError> {
    spawn_next_instance(&InstanceSpawn {
        elf: SNAKE_ELF,
        cert: SNAKE_NONOS_ID_CERT_BYTES,
        manifest: SNAKE_MANIFEST_BYTES,
        attestation: SNAKE_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::CoreExec.bit()
            | Capability::IPC.bit()
            | Capability::Memory.bit()
            | Capability::GraphicsDisplayQuery.bit()
            | Capability::GraphicsSurfaceCreate.bit(),
        instances: SNAKE_INSTANCES,
        debug_tag: b"[SNAKE-INSTANCE] elf error:",
    })
}

pub fn spawn_snake_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: SNAKE_ELF,
        nonos_id_cert_bytes: SNAKE_NONOS_ID_CERT_BYTES,
        manifest_bytes: SNAKE_MANIFEST_BYTES,
        attestation_trailer: SNAKE_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::CoreExec.bit()
            | Capability::IPC.bit()
            | Capability::Memory.bit()
            | Capability::GraphicsDisplayQuery.bit()
            | Capability::GraphicsSurfaceCreate.bit(),
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
