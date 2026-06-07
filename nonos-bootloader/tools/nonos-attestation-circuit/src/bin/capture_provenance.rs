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

use nonos_attestation_circuit::{
    compute_build_config_hash, compute_cargo_lock_hash, compute_rustc_version_hash,
    compute_source_tree_hash, BuildProvenance,
};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn get_git_commit_hash() -> Result<[u8; 32], String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("failed to run git: {}", e))?;

    if !output.status.success() {
        return Err("git rev-parse failed".into());
    }

    let commit = String::from_utf8_lossy(&output.stdout);
    let commit = commit.trim();

    if commit.len() < 40 {
        return Err("invalid git commit hash".into());
    }

    let mut hash = [0u8; 32];
    for (i, chunk) in commit.as_bytes().chunks(2).take(32).enumerate() {
        if chunk.len() == 2 {
            hash[i] =
                u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16).unwrap_or(0);
        }
    }

    Ok(hash)
}

fn get_git_tree_hash() -> Result<[u8; 32], String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD^{tree}"])
        .output()
        .map_err(|e| format!("failed to run git: {}", e))?;

    if !output.status.success() {
        return Err("git rev-parse tree failed".into());
    }

    let tree = String::from_utf8_lossy(&output.stdout);
    let tree = tree.trim();

    if tree.len() < 40 {
        return Err("invalid git tree hash".into());
    }

    let mut hash = [0u8; 32];
    for (i, chunk) in tree.as_bytes().chunks(2).take(32).enumerate() {
        if chunk.len() == 2 {
            hash[i] =
                u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16).unwrap_or(0);
        }
    }

    Ok(hash)
}

fn get_rustc_version() -> Result<(String, [u8; 32]), String> {
    let output = Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()
        .map_err(|e| format!("failed to run rustc: {}", e))?;

    if !output.status.success() {
        return Err("rustc --version failed".into());
    }

    let version_info = String::from_utf8_lossy(&output.stdout);
    let version_string = version_info.lines().next().unwrap_or("").to_string();

    let mut commit_hash = [0u8; 32];
    for line in version_info.lines() {
        if line.starts_with("commit-hash:") {
            let hash_str = line.trim_start_matches("commit-hash:").trim();
            for (i, chunk) in hash_str.as_bytes().chunks(2).take(32).enumerate() {
                if chunk.len() == 2 {
                    commit_hash[i] =
                        u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or("00"), 16)
                            .unwrap_or(0);
                }
            }
            break;
        }
    }

    Ok((version_string, commit_hash))
}

fn find_cargo_lock(start_dir: &Path) -> Result<Vec<u8>, String> {
    let mut dir = start_dir;
    loop {
        let cargo_lock = dir.join("Cargo.lock");
        if cargo_lock.exists() {
            return fs::read(&cargo_lock).map_err(|e| format!("failed to read Cargo.lock: {}", e));
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return Err("Cargo.lock not found".into()),
        }
    }
}

fn find_cargo_toml(start_dir: &Path) -> Result<Vec<u8>, String> {
    let mut dir = start_dir;
    loop {
        let cargo_toml = dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return fs::read(&cargo_toml).map_err(|e| format!("failed to read Cargo.toml: {}", e));
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return Err("Cargo.toml not found".into()),
        }
    }
}

fn find_cargo_config(start_dir: &Path) -> Vec<u8> {
    let mut dir = start_dir;
    loop {
        let config_toml = dir.join(".cargo").join("config.toml");
        if config_toml.exists() {
            if let Ok(contents) = fs::read(&config_toml) {
                return contents;
            }
        }
        let config = dir.join(".cargo").join("config");
        if config.exists() {
            if let Ok(contents) = fs::read(&config) {
                return contents;
            }
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return Vec::new(),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: capture_provenance <output_path>");
        std::process::exit(1);
    }

    let output_path = Path::new(&args[1]);
    let cwd = std::env::current_dir().expect("failed to get current directory");

    eprintln!("capturing build provenance...");

    let git_commit = match get_git_commit_hash() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let git_tree = match get_git_tree_hash() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };

    let source_tree_hash = compute_source_tree_hash(&git_commit, &git_tree);
    eprintln!("  source tree hash: {}", hex::encode(source_tree_hash));

    let cargo_lock = match find_cargo_lock(&cwd) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
    let cargo_lock_hash = compute_cargo_lock_hash(&cargo_lock);
    eprintln!("  cargo lock hash: {}", hex::encode(cargo_lock_hash));

    let (rustc_version, rustc_commit) = match get_rustc_version() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
    let rustc_version_hash = compute_rustc_version_hash(rustc_version.as_bytes(), &rustc_commit);
    eprintln!("  rustc version: {}", rustc_version);
    eprintln!("  rustc version hash: {}", hex::encode(rustc_version_hash));

    let cargo_toml = match find_cargo_toml(&cwd) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
    let cargo_config = find_cargo_config(&cwd);
    let build_config_hash = compute_build_config_hash(&cargo_toml, &cargo_config);
    eprintln!("  build config hash: {}", hex::encode(build_config_hash));

    let provenance = BuildProvenance::new(
        source_tree_hash,
        cargo_lock_hash,
        rustc_version_hash,
        build_config_hash,
    );

    let composite_hash = provenance.compute_composite_hash();
    eprintln!("  composite provenance hash: {}", hex::encode(composite_hash));

    let provenance_bytes = provenance.to_bytes();

    let mut output = fs::File::create(output_path).expect("failed to create output file");
    output.write_all(&provenance_bytes).expect("failed to write provenance bytes");
    output.write_all(&composite_hash).expect("failed to write composite hash");

    eprintln!("wrote provenance to {}", output_path.display());
    eprintln!("  total bytes: {}", provenance_bytes.len() + composite_hash.len());
}
