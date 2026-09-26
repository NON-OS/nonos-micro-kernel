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

//! Admitting the Linux personality through the same verified path every
//! capsule takes.

use super::embed::{
    LINUX_ATTESTATION_BYTES, LINUX_ELF, LINUX_MANIFEST_BYTES, LINUX_NONOS_ID_CERT_BYTES,
};
use super::state;
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "app.linux";
const SERVICE_PORT: u32 = 4936;
const REPLY_INBOX: &str = "endpoint.app.linux.reply";
const REPLY_PORT: u32 = 4937;
const TARGET_TRIPLE: &str = env!("NONOS_USER_TARGET");

/// Declared once here and mirrored by the capsule's manifest, which is what
/// the gate actually enforces.
pub const LINUX_CAPS: u64 = Capability::CoreExec.bit()
    | Capability::IPC.bit()
    | Capability::Memory.bit()
    | Capability::Crypto.bit()
    | Capability::Debug.bit()
    | Capability::ForeignExec.bit()
    | Capability::LocalSign.bit();

pub fn spawn_linux_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: LINUX_ELF,
        nonos_id_cert_bytes: LINUX_NONOS_ID_CERT_BYTES,
        manifest_bytes: LINUX_MANIFEST_BYTES,
        attestation_trailer: LINUX_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: LINUX_CAPS,
        debug_tag: b"",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    state::set_alive(pid);
    Ok(())
}
