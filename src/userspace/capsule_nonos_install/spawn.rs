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
    NONOS_INSTALL_ATTESTATION_BYTES, NONOS_INSTALL_ELF, NONOS_INSTALL_MANIFEST_BYTES,
    NONOS_INSTALL_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "app.nonos_install";
const SERVICE_PORT: u32 = 4860;
const REPLY_INBOX: &str = "endpoint.app.nonos_install.reply";
const REPLY_PORT: u32 = 4861;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

// The install ritual surveys hardware through the broker and reads the
// recorded boot verdict; it draws nothing and owns no window. DeviceEnum
// is the one grant beyond the console baseline, and it stays the whole
// justification for this capset until a step lands that needs more.
pub fn spawn_nonos_install_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: NONOS_INSTALL_ELF,
        nonos_id_cert_bytes: NONOS_INSTALL_NONOS_ID_CERT_BYTES,
        manifest_bytes: NONOS_INSTALL_MANIFEST_BYTES,
        attestation_trailer: NONOS_INSTALL_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: Capability::CoreExec.bit()
            | Capability::IO.bit()
            | Capability::IPC.bit()
            | Capability::Memory.bit()
            | Capability::DeviceEnum.bit(),
        debug_tag: b"[NONOS-INSTALL] elf error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
