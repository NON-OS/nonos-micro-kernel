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

//! The personality, spawned to install a package rather than host one.

use alloc::string::String;
use alloc::vec;

use super::embed::{
    LINUX_ATTESTATION_BYTES, LINUX_ELF, LINUX_MANIFEST_BYTES, LINUX_NONOS_ID_CERT_BYTES,
};
use super::spawn::LINUX_CAPS;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

// A second service name, because the installer is a second live process and
// two of them announcing one endpoint is a race over which answers.
const SERVICE_NAME: &str = "app.linux.install";
const SERVICE_PORT: u32 = 4938;
const REPLY_INBOX: &str = "endpoint.app.linux.install.reply";
const REPLY_PORT: u32 = 4939;

pub fn spawn_install(package: &str) -> Result<u32, SpawnError> {
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
        target_triple: env!("NONOS_USER_TARGET"),
        requested_caps: LINUX_CAPS,
        debug_tag: b"[LINUX-INSTALL] elf error:",
    };
    let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    let argv = vec![String::from("install"), String::from(package)];
    crate::process::with_process(pid, |pcb| *pcb.argv.lock() = argv);
    Ok(pid)
}
