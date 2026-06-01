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

use std::path::Path;
use std::process::Command;

use crate::util::load_manifest;

pub fn run(args: &[String]) -> Result<(), String> {
    let dir = args.first().map(String::as_str).unwrap_or(".");
    let m = load_manifest(Path::new(dir))?;
    let target_json =
        std::env::var("NONOS_TARGET_JSON").unwrap_or_else(|_| format!("{}.json", m.target));
    let mut cmd = Command::new("cargo");
    cmd.current_dir(dir)
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg(&target_json)
        .arg("-Zbuild-std=core,alloc")
        .arg("-Zbuild-std-features=compiler-builtins-mem");
    if let Ok(rustc) = std::env::var("NONOS_RUSTC") {
        cmd.env("RUSTC", rustc);
    }
    let status = cmd.status().map_err(|e| format!("launch cargo: {e}"))?;
    if !status.success() {
        return Err("cargo build failed".to_string());
    }
    println!("built {} ({target_json})", m.name);
    Ok(())
}
