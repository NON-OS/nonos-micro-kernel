use crate::report::{Report, Status};
use crate::sh::capture;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("reproducible", true);
    let out = Path::new(root).join("reproducible");
    let work = Path::new("target").join("repro-worktrees");
    let target = std::env::var("NONOS_REPRO_TARGET").unwrap_or_else(|_| "nonos-mk-esp".to_string());
    std::fs::create_dir_all(&out)?;
    std::fs::create_dir_all(&work)?;
    let clean =
        clean_tree() || std::env::var("NONOS_REPRO_ALLOW_DIRTY").ok().as_deref() == Some("1");
    rpt.check("clean-tree", st(clean), "git tree clean or explicitly allowed");
    if !clean {
        return rpt.finish(root);
    }

    let commit = capture("git", &["rev-parse", "HEAD"]).1.trim().to_string();
    let epoch = std::env::var("SOURCE_DATE_EPOCH")
        .unwrap_or_else(|_| capture("git", &["log", "-1", "--format=%ct"]).1.trim().to_string());
    let artifacts = artifacts();
    let a = build_lane("A", &work, &out, &commit, &epoch, &target, &artifacts)?;
    let b = build_lane("B", &work, &out, &commit, &epoch, &target, &artifacts)?;
    let same = a == b && !a.iter().any(|(_, h)| h == "MISSING");
    std::fs::write(out.join("hashes-A.json"), serde_json::to_string_pretty(&a).unwrap())?;
    std::fs::write(out.join("hashes-B.json"), serde_json::to_string_pretty(&b).unwrap())?;
    rpt.check(
        "double-build-hashes",
        st(same),
        "two clean worktree builds have identical artifact hashes",
    );
    rpt.finish(root)
}

fn clean_tree() -> bool {
    capture("git", &["status", "--porcelain"]).1.trim().is_empty()
}

fn artifacts() -> Vec<String> {
    std::env::var("NONOS_REPRO_ARTIFACTS")
        .unwrap_or_else(|_| {
            "target/x86_64-nonos/release/nonos-kernel target/kernel_signed.bin target/kernel_attested.bin nonos-bootloader/target/x86_64-unknown-uefi/release/nonos_boot.efi target/esp/EFI/Boot/BOOTX64.EFI target/esp/EFI/nonos/kernel.bin".to_string()
        })
        .split_whitespace()
        .map(str::to_string)
        .collect()
}

fn build_lane(
    lane: &str,
    work_root: &Path,
    out: &Path,
    commit: &str,
    epoch: &str,
    target: &str,
    artifacts: &[String],
) -> std::io::Result<Vec<(String, String)>> {
    let dir = work_root.join(format!("lane-{lane}"));
    let _ = Command::new("git").args(["worktree", "remove", "--force"]).arg(&dir).output();
    let add =
        Command::new("git").args(["worktree", "add", "--detach"]).arg(&dir).arg(commit).output()?;
    std::fs::write(out.join(format!("worktree-{lane}.log")), join(&add.stdout, &add.stderr))?;
    if !add.status.success() {
        return Ok(missing_hashes(artifacts));
    }
    // A worktree starts with empty submodule mounts; the build needs them.
    let sub = Command::new("git")
        .current_dir(&dir)
        .args(["submodule", "update", "--init", "--recursive"])
        .output()?;
    std::fs::write(out.join(format!("submodules-{lane}.log")), join(&sub.stdout, &sub.stderr))?;
    if !sub.status.success() {
        return Ok(missing_hashes(artifacts));
    }
    let mut cmd = Command::new("make");
    cmd.current_dir(&dir).arg(target).env("SOURCE_DATE_EPOCH", epoch).env("CARGO_INCREMENTAL", "0");
    if let Some(signing) = signing_key()? {
        cmd.env("SIGNING_KEY", signing);
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
    let hashes = hash_artifacts(&dir, artifacts)?;
    let _ = Command::new("git").args(["worktree", "remove", "--force"]).arg(&dir).output();
    Ok(hashes)
}

fn signing_key() -> std::io::Result<Option<PathBuf>> {
    if let Ok(path) = std::env::var("SIGNING_KEY") {
        return Ok(Some(PathBuf::from(path)));
    }
    let path = std::env::current_dir()?.join(".keys/signing_key_v1.bin");
    Ok(path.exists().then_some(path))
}

fn hash_artifacts(root: &Path, artifacts: &[String]) -> std::io::Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    for rel in artifacts {
        let path = root.join(rel);
        let hash = if path.exists() {
            blake3::hash(&std::fs::read(path)?).to_hex().to_string()
        } else {
            "MISSING".to_string()
        };
        out.push((rel.clone(), hash));
    }
    Ok(out)
}

fn missing_hashes(artifacts: &[String]) -> Vec<(String, String)> {
    artifacts.iter().map(|a| (a.clone(), "MISSING".to_string())).collect()
}

fn join(stdout: &[u8], stderr: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(stdout.len() + stderr.len());
    buf.extend_from_slice(stdout);
    buf.extend_from_slice(stderr);
    buf
}

fn st(ok: bool) -> Status {
    if ok {
        Status::Pass
    } else {
        Status::Fail
    }
}
