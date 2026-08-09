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
    INSTALLER_ATTESTATION_BYTES, INSTALLER_ELF, INSTALLER_MANIFEST_BYTES,
    INSTALLER_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "installer";
const SERVICE_PORT: u32 = 4112;
const REPLY_INBOX: &str = "endpoint.4294967322";
const REPLY_PORT: u32 = 4113;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");
// CoreExec | IPC | Memory | FileSystem | SpawnBroker; kept hand-synced with
// userland/capsule_installer/Capsule.mk's CAPSULE_REQUIRED_CAPS.
const REQUIRED_CAPS: u64 = 0x800059;

pub fn spawn_installer_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: INSTALLER_ELF,
        nonos_id_cert_bytes: INSTALLER_NONOS_ID_CERT_BYTES,
        manifest_bytes: INSTALLER_MANIFEST_BYTES,
        attestation_trailer: INSTALLER_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: REQUIRED_CAPS,
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
