// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display names for capability bits, hand-synced with the kernel's
//! `capabilities/types/{bit,as_str}.rs` (bit i == 1 << i, names verbatim). A
//! set bit this table does not name is still spelled out, as `bit<N>`: a
//! consent screen must never hide a capability that was granted.

use alloc::vec::Vec;

use crate::server::handlers::pkg_install::push_i32;

const NAMES: [&[u8]; 27] = [
    b"CoreExec",
    b"IO",
    b"Network",
    b"IPC",
    b"Memory",
    b"Crypto",
    b"FileSystem",
    b"Hardware",
    b"Debug",
    b"Admin",
    b"RegisterService",
    b"GraphicsDisplayQuery",
    b"GraphicsSurfaceCreate",
    b"GraphicsSurfaceMap",
    b"GraphicsPresent",
    b"DeviceEnum",
    b"Driver",
    b"Mmio",
    b"Irq",
    b"Dma",
    b"Pio",
    b"InputSource",
    b"TimeSet",
    b"SpawnBroker",
    b"SpawnWindow",
    b"ProcessControl",
    b"StoreWrite",
];

pub(super) fn append(caps: u64, out: &mut Vec<u8>) {
    let start = out.len();
    for i in 0..64 {
        if caps & (1u64 << i) == 0 {
            continue;
        }
        if out.len() != start {
            out.extend_from_slice(b", ");
        }
        match NAMES.get(i) {
            Some(name) => out.extend_from_slice(name),
            None => {
                out.extend_from_slice(b"bit");
                push_i32(out, i as i32);
            }
        }
    }
    if out.len() == start {
        out.extend_from_slice(b"(none)");
    }
}
