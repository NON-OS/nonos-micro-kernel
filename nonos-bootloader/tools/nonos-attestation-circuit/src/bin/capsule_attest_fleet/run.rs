// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use clap::Parser;

use super::args::Args;
use super::discover::discover;
use super::map_by_name::map_by_name;
use super::print_header::print_header;
use super::print_pass::print_pass;
use super::print_summary::print_summary;
use super::require_same_names::require_same_names;
use super::tool_path::tool_path;
use super::verify_one::verify_one;

pub fn run() -> Result<(), String> {
    let args = Args::parse();
    let capsules = discover(&args.capsule_dir, ".capsule.zk")?;
    let sidecars = discover(&args.sidecar_dir, ".zk_trailer.bin")?;
    if capsules.is_empty() {
        return Err("no capsule attestation blobs found".into());
    }
    let capsule_map = map_by_name(&capsules, ".capsule.zk")?;
    let sidecar_map = map_by_name(&sidecars, ".zk_trailer.bin")?;
    require_same_names(&capsule_map, &sidecar_map)?;
    let verifier = tool_path("verify-proof")?;
    print_header();
    for (name, capsule) in &capsule_map {
        verify_one(&verifier, &args.verifying_key, capsule)?;
        let sidecar = sidecar_map.get(name).ok_or("missing sidecar")?;
        let bytes = std::fs::metadata(sidecar).map_err(|e| format!("stat sidecar: {e}"))?.len();
        print_pass(name, bytes);
    }
    print_summary(capsule_map.len(), sidecar_map.len());
    Ok(())
}
