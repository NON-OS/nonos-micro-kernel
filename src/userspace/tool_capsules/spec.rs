// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! One embedded tool capsule and how it spawns. The identity and the signed,
//! attested artifacts are data; the capability set and trust anchor are the same
//! for every tool, so a tool is a value, not a hand-written module.

extern crate alloc;

use crate::capabilities::Capability;
use crate::kernel_core::process_spawn::capsule_spawn::{self, CapsuleSpecVerified, SpawnError};
use crate::security::nonos_id_cert::IdCertVerifyError;
use crate::security::nonos_trust_anchor::{
    decode as decode_trust_anchor, BAKED_TRUST_ANCHOR_POLICY,
};

/// A tool capsule baked into the kernel image: its endpoint identity and the
/// signed cert, manifest, and STARK attestation trailer the spawner verifies.
pub struct ToolCapsule {
    pub name: &'static str,
    pub service_port: u32,
    pub reply_inbox: &'static str,
    pub reply_port: u32,
    pub elf: &'static [u8],
    pub cert: &'static [u8],
    pub manifest: &'static [u8],
    pub attestation: &'static [u8],
}

impl ToolCapsule {
    /// Verify the artifacts under the baked trust anchor and spawn the tool with
    /// the sandboxed capability set (execute, IPC, memory) every tool shares.
    /// The process is parented to the caller in the syscall context, so that
    /// caller feeds its stdin and drains its stdout, and `argv` (a NUL-separated
    /// `name\0arg1\0...` blob) becomes the tool's argument vector. Returns the
    /// tool's pid.
    pub fn spawn_with_args(&self, argv: &[u8]) -> Result<u32, SpawnError> {
        let trust_anchor = decode_trust_anchor(BAKED_TRUST_ANCHOR_POLICY)
            .map_err(|_| SpawnError::NonosIdCertRejected(IdCertVerifyError::TrustAnchorPolicy))?;
        let spec = CapsuleSpecVerified {
            name: self.name,
            service_port: self.service_port,
            reply_inbox: self.reply_inbox,
            reply_port: self.reply_port,
            elf: self.elf,
            nonos_id_cert_bytes: self.cert,
            manifest_bytes: self.manifest,
            attestation_trailer: self.attestation,
            target_triple: env!("NONOS_USER_TARGET"),
            requested_caps: Capability::CoreExec.bit()
                | Capability::IPC.bit()
                | Capability::Memory.bit(),
            debug_tag: b"",
        };
        let pid = capsule_spawn::spawn_verified(&spec, &trust_anchor, None)?;
        if !argv.is_empty() {
            let vec: alloc::vec::Vec<alloc::string::String> = argv
                .split(|&b| b == 0)
                .filter(|s| !s.is_empty())
                .map(|s| alloc::string::String::from_utf8_lossy(s).into_owned())
                .collect();
            crate::process::with_process(pid, |pcb| *pcb.argv.lock() = vec);
        }
        Ok(pid)
    }
}
