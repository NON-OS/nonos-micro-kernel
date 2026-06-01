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

use std::fs;
use std::path::Path;

use crate::templates::{CARGO_TOML, MAIN_RS, NONOS_TOML, TOOLCHAIN};
use crate::util::write_file;

pub fn run(args: &[String]) -> Result<(), String> {
    let name = args.first().ok_or("usage: new <name>")?;
    let sdk = std::env::var("NONOS_SDK_PATH").unwrap_or_else(|_| "../sdk/nonos_sdk".to_string());
    let root = Path::new(name);
    fs::create_dir_all(root.join("src")).map_err(|e| format!("mkdir: {e}"))?;
    let cargo = CARGO_TOML.replace("__NAME__", name).replace("__SDK_PATH__", &sdk);
    write_file(&root.join("Cargo.toml"), &cargo)?;
    write_file(&root.join("src/main.rs"), &MAIN_RS.replace("__NAME__", name))?;
    write_file(&root.join("Nonos.toml"), &NONOS_TOML.replace("__NAME__", name))?;
    write_file(&root.join("rust-toolchain.toml"), TOOLCHAIN)?;
    println!("created capsule project `{name}` (edit Nonos.toml caps + keys, then `nonos-capsule build`)");
    Ok(())
}
