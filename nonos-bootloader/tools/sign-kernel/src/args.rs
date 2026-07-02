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
    name = "nonos-sign-kernel",
    about = "Sign NONOS kernel binary with Ed25519 and ML-DSA-65"
)]
pub struct Args {
    #[arg(short, long, value_name = "FILE", conflicts_with = "vault_addr")]
    pub key: Option<PathBuf>,
    #[arg(long, value_name = "FILE")]
    pub mldsa65_key: Option<PathBuf>,
    #[arg(long, value_name = "FILE")]
    pub mldsa65_pub: Option<PathBuf>,
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
    #[arg(long, value_name = "URL")]
    pub vault_addr: Option<String>,
    #[arg(long, value_name = "TOKEN", env = "VAULT_TOKEN")]
    pub vault_token: Option<String>,
    #[arg(long, value_name = "NAME", default_value = "nonos-kernel-signing")]
    pub vault_key_name: String,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub verify: bool,
    #[arg(long, value_name = "N", default_value_t = 0)]
    pub rollback_index: u32,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,
}
