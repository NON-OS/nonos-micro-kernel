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

pub const CARGO_TOML: &str = r#"[package]
name = "__NAME__"
version = "0.1.0"
edition = "2021"
publish = false
license = "AGPL-3.0"

[[bin]]
name = "__NAME__"
path = "src/main.rs"

[dependencies]
nonos-sdk = { path = "__SDK_PATH__" }

[profile.release]
panic = "abort"
opt-level = 2
lto = false
strip = true
"#;

pub const MAIN_RS: &str = r#"#![no_std]
#![no_main]

use nonos_sdk::prelude::*;

fn app() {
    let _ = App::new("__NAME__").window().show();
}

sdk_main!(app);
"#;

pub const NONOS_TOML: &str = r#"name = "__NAME__"
namespace = "systems.nonos.__NAME__"
version = "0.1.0"
target = "x86_64-nonos-user"
cert = "keys/publisher.cert"
required_caps = ["CoreExec", "Memory", "GraphicsDisplayQuery", "GraphicsSurfaceCreate", "GraphicsSurfaceMap", "GraphicsPresent"]
optional_caps = []
pub_seed_ed25519 = "keys/ed25519.seed"
pub_seed_mldsa65 = "keys/mldsa65.seed"
"#;

pub const TOOLCHAIN: &str = "[toolchain]\nchannel = \"nightly-2026-01-16\"\n";
