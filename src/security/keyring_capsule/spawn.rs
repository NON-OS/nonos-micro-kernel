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

use super::client::REPLY_INBOX;
use super::embed::{
    KEYRING_ATTESTATION_BYTES, KEYRING_ELF, KEYRING_MANIFEST_BYTES, KEYRING_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

pub use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

const SERVICE_NAME: &str = "keyring";
const SERVICE_PORT: u32 = 4098;
const REPLY_PORT: u32 = 4099;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

// The capsule itself never needs CAP_KEYRING — it is the keyring.
// Bits below are what the userland binary needs to run: IPC for
// MkIpc recv/send, Memory for heap_init/mmap, Crypto for any
// kernel crypto syscall the implementation may reach. Manifest is
// the source of truth at spawn time.
pub fn spawn_keyring_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;

    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: KEYRING_ELF,
        nonos_id_cert_bytes: KEYRING_NONOS_ID_CERT_BYTES,
        manifest_bytes: KEYRING_MANIFEST_BYTES,
        attestation_trailer: KEYRING_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::IPC.bit() | Capability::Memory.bit() | Capability::Crypto.bit(),
        debug_tag: b"[KEYRING-DEBUG] load_elf_executable error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
