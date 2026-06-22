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
use nonos_attestation_circuit::transparent::verify;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    ctx: PathBuf,
    #[arg(long)]
    proof: PathBuf,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let root_bytes = fs::read(&args.root).map_err(|e| e.to_string())?;
    let root: [u8; 32] =
        root_bytes.get(..32).ok_or("root too short")?.try_into().map_err(|_| "root")?;
    let ctx = fs::read(&args.ctx).map_err(|e| e.to_string())?;
    let proof = fs::read(&args.proof).map_err(|e| e.to_string())?;
    verify(&root, &ctx, &proof)?;
    println!("RESULT: PASS");
    Ok(())
}
