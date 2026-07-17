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
#[command(
    name = "embed-zk-proof",
    about = "Generate and embed transparent ZK attestation into signed NONOS kernel"
)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,

    // A pre-made attestation trailer to embed verbatim as the proof region. When
    // set, the curve enrolled-secret proof below is not generated; this is how the
    // kernel's transparent STARK self-attestation trailer is carried in the image.
    #[arg(long, value_name = "FILE")]
    pub proof_file: Option<PathBuf>,

    // The curve enrolled-secret arguments. Defaulted so they are optional when
    // --proof-file is used; the curve path requires the real values.
    #[arg(long, value_name = "FILE", default_value = "")]
    pub root: PathBuf,

    #[arg(long, default_value = "")]
    pub secret_x: String,

    #[arg(long, default_value = "")]
    pub secret_r: String,

    #[arg(long, value_name = "FILE", default_value = "")]
    pub commitments: PathBuf,

    #[arg(long, default_value_t = 0)]
    pub index: usize,

    #[arg(long)]
    pub boot_nonce: Option<String>,

    #[arg(long)]
    pub machine_id: Option<String>,

    #[arg(long)]
    pub timestamp: Option<u64>,

    #[arg(long, value_name = "FILE")]
    pub challenge: Option<PathBuf>,

    #[arg(long, default_value = "")]
    pub nonce_seed: String,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub sidecar: bool,
}
