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

use std::fs;

use clap::Parser;

use super::args::Args;
use super::default_capsule::default_capsule;
use super::ensure_capsule::ensure_capsule;
use super::generate_keys::generate_keys;
use super::generate_proof::generate_proof;
use super::print_header::print_header;
use super::print_proof_lines::print_proof_lines;
use super::require_tool::require_tool;
use super::temp_root::temp_root;
use super::tool_path::tool_path;
use super::verify_tampered::verify_tampered;
use super::verify_valid::verify_valid;

pub fn run() -> Result<(), String> {
    let args = Args::parse();
    let capsule = ensure_capsule(args.capsule.unwrap_or_else(default_capsule))?;
    let len = fs::metadata(&capsule).map_err(|e| format!("stat capsule: {e}"))?.len();
    let root = temp_root()?;
    let keys = root.join("keys");
    let keygen = tool_path("generate-keys")?;
    let prover = tool_path("generate-proof")?;
    let verifier = tool_path("verify-proof")?;
    require_tool(&keygen)?;
    require_tool(&prover)?;
    require_tool(&verifier)?;
    print_header(&capsule, len);
    println!("  $ generate-keys generate --allow-unsigned");
    generate_keys(&keygen, &keys)?;
    println!("  keys ready\n");
    println!("  $ generate-proof --capsule terminal --capability-mask {}", args.capability_mask);
    let proof_out = generate_proof(
        &prover,
        &keys.join("attestation_proving_key.bin"),
        &capsule,
        &args.capability_mask,
        &root,
    )?;
    print_proof_lines(&proof_out);
    let vk = keys.join("attestation_verifying_key.bin");
    verify_valid(&verifier, &vk, &root.join("term.cap"))?;
    verify_tampered(&verifier, &vk, &root.join("term.cap"), &root.join("term_bad.cap"))?;
    Ok(())
}
