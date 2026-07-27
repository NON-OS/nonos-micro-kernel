// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tool capsules are signed, STARK-attested crates.io utilities baked into the
//! kernel image from `userland/apps.list`. Each carries its ELF, NONOS-ID cert,
//! manifest, and STARK trailer; the spawner verifies all four under the baked
//! trust anchor before it runs. `spawn_all` brings them up after the desktop.

#[macro_use]
mod embed_macro;
mod registry;
mod spec;

pub use registry::spawn_all;
