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
use nonos_attestation_circuit::transparent::{commitment, enroll_secret, root};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    seed: String,
    #[arg(long)]
    labels: PathBuf,
    #[arg(long)]
    root_out: PathBuf,
    #[arg(long)]
    secrets_out: PathBuf,
    #[arg(long)]
    commitments_out: PathBuf,
    #[arg(long)]
    fixed_depth: Option<u8>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let labels = fs::read_to_string(&args.labels).map_err(|e| e.to_string())?;
    let mut commitments = Vec::new();
    let mut commitments_raw = Vec::new();
    let mut secrets = String::new();
    for label in labels.lines().filter(|l| !l.trim().is_empty()) {
        let secret = enroll_secret(label.as_bytes(), args.seed.as_bytes());
        let c = commitment(&secret);
        commitments.push(c);
        commitments_raw.extend_from_slice(&c);
        secrets.push_str(label);
        secrets.push(' ');
        secrets.push_str(&hex::encode(secret.x.to_bytes()));
        secrets.push(' ');
        secrets.push_str(&hex::encode(secret.r.to_bytes()));
        secrets.push('\n');
    }
    if let Some(depth) = args.fixed_depth {
        let target = 1usize.checked_shl(u32::from(depth)).ok_or("fixed depth too large")?;
        if commitments.len() > target {
            return Err("too many labels for fixed depth".into());
        }
        for index in commitments.len()..target {
            let label = format!("__nonos_pad_{index}");
            let secret = enroll_secret(label.as_bytes(), args.seed.as_bytes());
            let c = commitment(&secret);
            commitments.push(c);
            commitments_raw.extend_from_slice(&c);
        }
    }
    fs::write(&args.root_out, root(&commitments)).map_err(|e| e.to_string())?;
    fs::write(&args.secrets_out, secrets).map_err(|e| e.to_string())?;
    fs::write(&args.commitments_out, commitments_raw).map_err(|e| e.to_string())?;
    Ok(())
}
