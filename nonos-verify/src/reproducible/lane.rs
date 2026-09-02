// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

//! One build lane: a detached worktree of the commit under test, its
//! submodules, and one make invocation, with every log written beside
//! the report.

use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn build(
    lane: &str,
    work_root: &Path,
    out: &Path,
    commit: &str,
    epoch: &str,
    target: &str,
) -> std::io::Result<PathBuf> {
    let dir = work_root.join(format!("lane-{lane}"));
    remove(&dir);
    let add =
        Command::new("git").args(["worktree", "add", "--detach"]).arg(&dir).arg(commit).output()?;
    std::fs::write(out.join(format!("worktree-{lane}.log")), join(&add.stdout, &add.stderr))?;
    if !add.status.success() {
        return Ok(dir);
    }
    // A worktree starts with empty submodule mounts; the build needs them.
    let sub = Command::new("git")
        .current_dir(&dir)
        .args(["submodule", "update", "--init", "--recursive"])
        .output()?;
    std::fs::write(out.join(format!("submodules-{lane}.log")), join(&sub.stdout, &sub.stderr))?;
    if !sub.status.success() {
        return Ok(dir);
    }
    let mut cmd = Command::new("make");
    cmd.current_dir(&dir).arg(target).env("SOURCE_DATE_EPOCH", epoch).env("CARGO_INCREMENTAL", "0");
    if let Some(signing) = signing_key()? {
        cmd.env("SIGNING_KEY", signing);
    }
    // Both lanes must sign under the same ML-DSA keypair; left to the
    // Makefile default each lane mints its own and the baked public key
    // makes the two bootloaders differ.
    if let Some(prefix) = mldsa_prefix()? {
        cmd.env("KERNEL_MLDSA65_PREFIX", prefix);
    }
    for key in
        ["NONOS_DEV", "ZK_BOOT_INDEX", "ZK_BOOT_SECRET_X", "ZK_BOOT_SECRET_R", "ZK_BOOT_NONCE_SEED"]
    {
        if let Ok(value) = std::env::var(key) {
            cmd.env(key, value);
        }
    }
    let run = cmd.output()?;
    std::fs::write(out.join(format!("build-{lane}.log")), join(&run.stdout, &run.stderr))?;
    Ok(dir)
}

pub(super) fn remove(dir: &Path) {
    let _ = Command::new("git").args(["worktree", "remove", "--force"]).arg(dir).output();
}

fn signing_key() -> std::io::Result<Option<PathBuf>> {
    if let Ok(path) = std::env::var("SIGNING_KEY") {
        return Ok(Some(PathBuf::from(path)));
    }
    let path = std::env::current_dir()?.join(".keys/signing_key_v1.bin");
    Ok(path.exists().then_some(path))
}

fn mldsa_prefix() -> std::io::Result<Option<PathBuf>> {
    if let Ok(prefix) = std::env::var("KERNEL_MLDSA65_PREFIX") {
        return Ok(Some(PathBuf::from(prefix)));
    }
    let prefix = std::env::current_dir()?.join("nonos-bootloader/keys/kernel_mldsa65");
    let seeded = prefix.with_extension("seed").exists();
    Ok(seeded.then_some(prefix))
}

// The shared ML-DSA keypair lives in the main checkout under a
// gitignored path; minting it here keeps the tree clean and gives both
// lanes the same input.
pub(super) fn ensure_shared_keys(out: &Path) -> std::io::Result<()> {
    let run = Command::new("make").arg("nonos-mk-ensure-signing-key").output()?;
    std::fs::write(out.join("ensure-keys.log"), join(&run.stdout, &run.stderr))?;
    Ok(())
}

fn join(stdout: &[u8], stderr: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(stdout.len() + stderr.len());
    buf.extend_from_slice(stdout);
    buf.extend_from_slice(stderr);
    buf
}
