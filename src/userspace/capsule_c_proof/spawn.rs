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

use super::embed::C_PROOF_ELF;
#[cfg(not(feature = "nonos-dev-unverified-capsules"))]
use super::embed::{
    C_PROOF_ATTESTATION_BYTES, C_PROOF_MANIFEST_BYTES, C_PROOF_NONOS_ID_CERT_BYTES,
};
use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, SpawnError};
#[cfg(not(feature = "nonos-dev-unverified-capsules"))]
use crate::kernel_core::process_spawn::capsule_spawn::CapsuleSpecVerified;
#[cfg(feature = "nonos-dev-unverified-capsules")]
use crate::kernel_core::process_spawn::capsule_spawn::CapsuleSpec;
#[cfg(not(feature = "nonos-dev-unverified-capsules"))]
use crate::security::nonos_id_cert::IdCertVerifyError;
#[cfg(not(feature = "nonos-dev-unverified-capsules"))]
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

const SERVICE_NAME: &str = "c_proof";
const SERVICE_PORT: u32 = 4504;
const REPLY_INBOX: &str = "endpoint.c_proof.reply";
const REPLY_PORT: u32 = 4505;
#[cfg(not(feature = "nonos-dev-unverified-capsules"))]
const TARGET_TRIPLE: &str = "x86_64-nonos-user";

fn requested_caps() -> u64 {
    Capability::CoreExec.bit()
        | Capability::IPC.bit()
        | Capability::Memory.bit()
        | Capability::Debug.bit()
}

#[cfg(not(feature = "nonos-dev-unverified-capsules"))]
pub fn spawn_c_proof_capsule() -> Result<(), SpawnError> {
    let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
        .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;

    let spec = CapsuleSpecVerified {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: C_PROOF_ELF,
        nonos_id_cert_bytes: C_PROOF_NONOS_ID_CERT_BYTES,
        manifest_bytes: C_PROOF_MANIFEST_BYTES,
        attestation_trailer: C_PROOF_ATTESTATION_BYTES,
        target_triple: TARGET_TRIPLE,
        requested_caps: requested_caps(),
        debug_tag: b"",
    };
    capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
    Ok(())
}

#[cfg(feature = "nonos-dev-unverified-capsules")]
pub fn spawn_c_proof_capsule() -> Result<(), SpawnError> {
    let spec = CapsuleSpec {
        name: SERVICE_NAME,
        service_port: SERVICE_PORT,
        reply_inbox: REPLY_INBOX,
        reply_port: REPLY_PORT,
        elf: C_PROOF_ELF,
        caps_bits: requested_caps(),
        debug_tag: b"",
    };
    capsule_spawn::spawn(&spec)?;
    Ok(())
}
