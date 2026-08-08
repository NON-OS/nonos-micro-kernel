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
    NET_SOCKETS_ATTESTATION_BYTES, NET_SOCKETS_ELF, NET_SOCKETS_MANIFEST_BYTES,
    NET_SOCKETS_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

const SERVICE_NAME: &str = "net.sockets";
const SERVICE_PORT: u32 = 4460;
const REPLY_INBOX: &str = "endpoint.net.sockets.reply";
const REPLY_PORT: u32 = 4461;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub fn spawn_net_sockets_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: NET_SOCKETS_ELF,
        nonos_id_cert_bytes: NET_SOCKETS_NONOS_ID_CERT_BYTES,
        manifest_bytes: NET_SOCKETS_MANIFEST_BYTES,
        attestation_trailer: NET_SOCKETS_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::IPC.bit()
            | Capability::Memory.bit()
            | crate::capabilities::serial_debug_cap()
            | Capability::Network.bit(),
        debug_tag: b"[NET-SOCKETS] load_elf_executable error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
