// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The `tool_capsule!` macro: bake one tool's signed artifacts into the image.

/// Build a `ToolCapsule` from its endpoint identity and its binary name. The ELF
/// path is given explicitly; the cert, manifest, and attestation trailer are
/// found by binary name under the trust directory. All paths resolve relative to
/// the invoking file, which is the registry.
macro_rules! tool_capsule {
    ($name:literal, $port:expr, $reply:literal, $reply_port:expr, $elf:literal, $bin:literal) => {
        super::spec::ToolCapsule {
            name: $name,
            service_port: $port,
            reply_inbox: $reply,
            reply_port: $reply_port,
            elf: include_bytes!($elf),
            cert: include_bytes!(concat!(
                "../../../nonos-data/trust/capsules/",
                $bin,
                ".nonos_id_cert.bin"
            )),
            manifest: include_bytes!(concat!(
                "../../../nonos-data/trust/capsules/",
                $bin,
                ".manifest.bin"
            )),
            attestation: include_bytes!(concat!(
                "../../../nonos-data/trust/capsules/",
                $bin,
                ".zk_trailer.bin"
            )),
        }
    };
}
