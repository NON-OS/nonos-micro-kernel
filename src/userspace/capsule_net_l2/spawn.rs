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

//! Spawn the `net.l2` ethernet+ARP capsule.
//! The NIC-side authority (MMIO/IRQ/DMA) lives one layer below at
//! the virtio-net (or e1000) driver capsule; this service reaches
//! the NIC through the service registry, never through the kernel.

use super::embed::{
    NET_L2_ATTESTATION_BYTES, NET_L2_ELF, NET_L2_MANIFEST_BYTES, NET_L2_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

const SERVICE_NAME: &str = "net.l2";
const SERVICE_PORT: u32 = 4400;
const REPLY_INBOX: &str = "endpoint.net.l2.reply";
const REPLY_PORT: u32 = 4401;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

pub fn spawn_net_l2_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;

    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: NET_L2_ELF,
        nonos_id_cert_bytes: NET_L2_NONOS_ID_CERT_BYTES,
        manifest_bytes: NET_L2_MANIFEST_BYTES,
        attestation_trailer: NET_L2_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::IPC.bit()
            | Capability::Memory.bit()
            | Capability::Network.bit(),
        debug_tag: b"[NET-L2] load_elf_executable error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
