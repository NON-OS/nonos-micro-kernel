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

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "generate-proof", about = "Generate a NONOS capsule Groth16 proof")]
pub struct Args {
    #[arg(short = 'k', long, value_name = "FILE")]
    pub proving_key: PathBuf,
    #[arg(short = 'o', long, value_name = "FILE")]
    pub output: PathBuf,
    #[arg(long, value_name = "FILE")]
    pub public_inputs_out: Option<PathBuf>,
    #[arg(long, value_name = "FILE")]
    pub trailer_out: Option<PathBuf>,
    #[arg(long, value_name = "FILE")]
    pub capsule_with_trailer_out: Option<PathBuf>,
    #[arg(long, value_name = "FILE")]
    pub capsule: PathBuf,
    #[arg(long, value_name = "MASK")]
    pub capability_mask: String,
    #[arg(long, value_name = "HEX")]
    pub program_hash: Option<String>,
    #[arg(long, default_value = "nonos-capsule-attestation")]
    pub seed: String,
}
