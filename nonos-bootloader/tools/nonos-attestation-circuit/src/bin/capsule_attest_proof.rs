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
use nonos_attestation_circuit::capsule::*;
use nonos_attestation_circuit::transparent::{proof_path, prove, verify, EnrolledSecret};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    label: String,
    #[arg(long)]
    secrets: PathBuf,
    #[arg(long)]
    commitments: PathBuf,
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    capsule: PathBuf,
    #[arg(long)]
    capability_mask: String,
    #[arg(long)]
    nonce_seed: String,
    #[arg(long, default_value_t = 1)]
    epoch: u64,
    #[arg(long)]
    ctx_out: PathBuf,
    #[arg(long)]
    proof_out: PathBuf,
    #[arg(long)]
    trailer_out: PathBuf,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let root = read_root(&args.root)?;
    let commitments = read_commitments(&args.commitments)?;
    let elf = fs::read(&args.capsule).map_err(|e| e.to_string())?;
    let caps = parse_caps(&args.capability_mask)?;
    let ctx = capsule_ctx(&elf, caps, args.epoch);
    let (index, x, r) = read_secret(&args.secrets, &args.label)?;
    if index >= commitments.len() {
        return Err("capsule enrollment index outside commitments".into());
    }
    let secret = EnrolledSecret { x, r };
    let (siblings, dirs) = proof_path(&commitments, index);
    let proof = prove(&secret, &siblings, &dirs, &root, &ctx, args.nonce_seed.as_bytes())?;
    verify(&root, &ctx, &proof)?;
    let trailer = capsule_trailer(&proof)?;
    fs::write(&args.ctx_out, ctx).map_err(|e| e.to_string())?;
    fs::write(&args.proof_out, proof).map_err(|e| e.to_string())?;
    fs::write(&args.trailer_out, trailer).map_err(|e| e.to_string())
}
