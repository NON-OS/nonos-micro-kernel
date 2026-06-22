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
use curve25519_dalek::scalar::Scalar;
use nonos_attestation_circuit::transparent::{proof_path, prove, EnrolledSecret};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    secret_x: String,
    #[arg(long)]
    secret_r: String,
    #[arg(long)]
    commitments: PathBuf,
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    ctx: PathBuf,
    #[arg(long)]
    index: usize,
    #[arg(long)]
    nonce_seed: String,
    #[arg(long)]
    out: PathBuf,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let x = hex::decode(args.secret_x).map_err(|e| e.to_string())?;
    let r = hex::decode(args.secret_r).map_err(|e| e.to_string())?;
    let root_bytes = fs::read(&args.root).map_err(|e| e.to_string())?;
    let root: [u8; 32] =
        root_bytes.get(..32).ok_or("root too short")?.try_into().map_err(|_| "root")?;
    let ctx = fs::read(&args.ctx).map_err(|e| e.to_string())?;
    let raw = fs::read(&args.commitments).map_err(|e| e.to_string())?;
    if raw.len() % 32 != 0 {
        return Err("commitments file is not 32-byte aligned".into());
    }
    let commitments: Vec<[u8; 32]> =
        raw.chunks_exact(32).filter_map(|c| c.try_into().ok()).collect();
    if args.index >= commitments.len() {
        return Err("enrollment index out of range".into());
    }
    let x_bytes: [u8; 32] =
        x.get(..32).ok_or("secret_x too short")?.try_into().map_err(|_| "secret_x")?;
    let r_bytes: [u8; 32] =
        r.get(..32).ok_or("secret_r too short")?.try_into().map_err(|_| "secret_r")?;
    let secret = EnrolledSecret {
        x: Option::<Scalar>::from(Scalar::from_canonical_bytes(x_bytes))
            .ok_or("secret_x noncanonical")?,
        r: Option::<Scalar>::from(Scalar::from_canonical_bytes(r_bytes))
            .ok_or("secret_r noncanonical")?,
    };
    let (siblings, dirs) = proof_path(&commitments, args.index);
    let proof = prove(&secret, &siblings, &dirs, &root, &ctx, args.nonce_seed.as_bytes())?;
    fs::write(&args.out, proof).map_err(|e| e.to_string())
}
