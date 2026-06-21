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

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use clap::Parser;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde::Serialize;
use sha2::{Digest, Sha256};
use tempfile::NamedTempFile;
use zeroize::Zeroize;

#[derive(Parser, Debug)]
#[command(
    name = "nonos-keygen",
    about = "Generate Ed25519 signer keys for NONOS"
)]
struct Args {
    #[arg(short, long, default_value_t = 4)]
    count: usize,

    #[arg(short, long, value_name = "DIR", default_value = "keys")]
    out_dir: PathBuf,

    #[arg(long, value_name = "fmt", default_value = "hex")]
    format: String,

    #[arg(long, value_name = "PATH")]
    signers: Option<PathBuf>,

    #[arg(long)]
    threshold: Option<usize>,

    #[arg(long, default_value = "signer")]
    id_prefix: String,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    insecure_world_readable: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub_only: bool,

    #[arg(long, value_name = "STR")]
    operator: Option<String>,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    allow_write_secrets: bool,
}

#[derive(Serialize)]
struct SignerEntry {
    id: String,
    pubkey_hex: String,
    pubkey_sha256: String,
    pubkey_blake3: String,
}

#[derive(Serialize)]
struct SignersJson {
    threshold: usize,
    signers: Vec<SignerEntry>,
}

#[derive(Serialize)]
struct GenerationLog {
    tool: String,
    tool_version: String,
    rustc_version: String,
    cargo_version: String,
    commit: Option<String>,
    created_at_utc: String,
    host_fingerprint: String,
    operator_hash: Option<String>,
    key_count: usize,
    threshold: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.count == 0 {
        bail!("count must be >= 1");
    }
    let fmt = args.format.to_lowercase();
    if !["raw", "hex", "base64"].contains(&fmt.as_str()) {
        bail!("unsupported format: {} (supported: raw, hex, base64)", fmt);
    }

    if !args.pub_only && !args.allow_write_secrets {
        eprintln!("Secret files will NOT be written unless --allow-write-secrets is provided.");
        std::process::exit(2);
    }

    fs::create_dir_all(&args.out_dir)
        .with_context(|| format!("creating {}", args.out_dir.display()))?;

    let tool = "nonos-keygen".to_string();
    let tool_version = env!("CARGO_PKG_VERSION").to_string();
    let rustc_version =
        get_command_output("rustc", &["--version"]).unwrap_or_else(|_| "unknown".into());
    let cargo_version =
        get_command_output("cargo", &["--version"]).unwrap_or_else(|_| "unknown".into());
    let commit = git_commit_hash();
    let created_at = Utc::now().to_rfc3339();
    let hostname = hostname::get()
        .ok()
        .and_then(|s| s.into_string().ok())
        .unwrap_or_else(|| "unknown".into());
    let host_fingerprint = sha256_hex(hostname.as_bytes());

    let operator_hash = args.operator.as_ref().map(|op| sha256_hex(op.as_bytes()));

    let threshold = args.threshold.unwrap_or_else(|| (args.count / 2) + 1);

    let mut signers: Vec<SignerEntry> = Vec::with_capacity(args.count);

    for i in 1..=args.count {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let pub_bytes = verifying_key.to_bytes();
        let mut sec_bytes = signing_key.to_bytes().to_vec();

        let id = format!("{}{}", args.id_prefix, i);
        let pub_raw_path = args.out_dir.join(format!("{}.pub.raw", id));
        let pub_hex_path = args.out_dir.join(format!("{}.pub.hex", id));
        let pub_b64_path = args.out_dir.join(format!("{}.pub.b64", id));
        let sec_raw_path = args.out_dir.join(format!("{}.key", id));
        let sec_hex_path = args.out_dir.join(format!("{}.key.hex", id));
        let sec_b64_path = args.out_dir.join(format!("{}.key.b64", id));

        write_atomic(&pub_raw_path, &pub_bytes)?;
        write_atomic_text(&pub_hex_path, &hex::encode(pub_bytes))?;
        write_atomic_text(&pub_b64_path, &BASE64.encode(pub_bytes))?;

        if !args.pub_only {
            let mode = if args.insecure_world_readable {
                0o644
            } else {
                0o600
            };
            write_atomic(&sec_raw_path, &sec_bytes)?;
            set_mode_if_unix(&sec_raw_path, mode)?;
            write_atomic_text(&sec_hex_path, &hex::encode(&sec_bytes))?;
            set_mode_if_unix(&sec_hex_path, mode)?;
            write_atomic_text(&sec_b64_path, &BASE64.encode(&sec_bytes))?;
            set_mode_if_unix(&sec_b64_path, mode)?;
        }

        let sha256_hex_val = sha256_hex(&pub_bytes);
        let blake3_hex = blake3::hash(&pub_bytes).to_hex().to_string();

        sec_bytes.zeroize();

        signers.push(SignerEntry {
            id: id.clone(),
            pubkey_hex: hex::encode(pub_bytes),
            pubkey_sha256: sha256_hex_val,
            pubkey_blake3: blake3_hex,
        });

        match fmt.as_str() {
            "raw" => println!("{}: wrote pub raw -> {}", id, pub_raw_path.display()),
            "hex" => println!("{}: wrote pub hex -> {}", id, pub_hex_path.display()),
            "base64" => println!("{}: wrote pub b64 -> {}", id, pub_b64_path.display()),
            _ => {}
        }
    }

    if let Some(signers_path) = args.signers {
        let sj = SignersJson { threshold, signers };
        let mut out_path = signers_path;
        if out_path.is_relative() {
            out_path = args.out_dir.join(out_path);
        }
        let json = serde_json::to_vec_pretty(&sj)?;
        write_atomic(&out_path, &json)?;
        println!("Wrote signers.json -> {}", out_path.display());
    }

    let gen_log = GenerationLog {
        tool,
        tool_version,
        rustc_version,
        cargo_version,
        commit,
        created_at_utc: created_at,
        host_fingerprint,
        operator_hash,
        key_count: args.count,
        threshold,
    };
    let gen_json = serde_json::to_vec_pretty(&gen_log)?;
    let gen_path = args.out_dir.join("generation_log.json");
    write_atomic(&gen_path, &gen_json)?;
    println!("Wrote generation_log.json -> {}", gen_path.display());

    println!(
        "Generated {} signer keypairs in {}",
        args.count,
        args.out_dir.display()
    );
    println!("Reminder: protect secret key files and do NOT commit them to source control.");

    Ok(())
}

fn write_atomic(path: &PathBuf, data: &[u8]) -> Result<()> {
    let mut tmp = NamedTempFile::new_in(path.parent().unwrap_or(Path::new(".")))?;
    tmp.write_all(data)?;
    tmp.as_file().sync_all()?;
    tmp.persist(path)
        .with_context(|| format!("persisting to {}", path.display()))?;
    Ok(())
}

fn write_atomic_text(path: &PathBuf, text: &str) -> Result<()> {
    write_atomic(path, text.as_bytes())
}

#[cfg(unix)]
fn set_mode_if_unix(path: &PathBuf, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
        .with_context(|| format!("chmod {}", path.display()))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_mode_if_unix(_path: &PathBuf, _mode: u32) -> Result<()> {
    Ok(())
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn get_command_output(cmd: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(cmd).args(args).output()?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else {
        Err(anyhow::anyhow!("{} failed", cmd))
    }
}

fn git_commit_hash() -> Option<String> {
    if let Ok(out) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        if out.status.success() {
            if let Ok(s) = String::from_utf8(out.stdout) {
                return Some(s.trim().to_string());
            }
        }
    }
    None
}

