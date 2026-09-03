use crate::report::{Report, Status};
use crate::sh::{capture, capture_stdout};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("release", true);
    let out = Path::new(root).join("release");
    let bundle = out.join("bundle");
    std::fs::create_dir_all(&bundle)?;
    let built = Command::new("make").arg("nonos-mk-esp").output()?;
    std::fs::write(out.join("release-build.log"), join(&built.stdout, &built.stderr))?;
    rpt.check("release-build", st(built.status.success()), "make nonos-mk-esp");

    let manifest = collect(&bundle)?;
    std::fs::write(
        out.join("release-manifest.json"),
        serde_json::to_string_pretty(&manifest).unwrap(),
    )?;
    std::fs::write(out.join("SHA256SUMS"), sums(&manifest, "sha256"))?;
    std::fs::write(out.join("BLAKE3SUMS"), sums(&manifest, "blake3"))?;
    let provenance = provenance(&manifest);
    std::fs::write(
        out.join("provenance.json"),
        serde_json::to_string_pretty(&provenance).unwrap(),
    )?;
    let packed = Command::new("tar")
        .arg("-czf")
        .arg(out.join("nonos-release-bundle.tar.gz"))
        .arg("-C")
        .arg(&out)
        .arg("bundle")
        .output()?;
    std::fs::write(out.join("tar.log"), join(&packed.stdout, &packed.stderr))?;
    rpt.check("bundle-tar", st(packed.status.success()), "release bundle archive written");
    let complete = !manifest.is_empty() && manifest.iter().all(|m| m["status"] == "present");
    rpt.check(
        "artifact-manifest",
        st(complete),
        "release manifest contains every required artifact",
    );
    rpt.finish(root)
}

fn collect(bundle: &Path) -> std::io::Result<Vec<serde_json::Value>> {
    let mut rows = Vec::new();
    for rel in artifacts() {
        let src = Path::new(&rel);
        if !src.exists() {
            rows.push(row(&rel, "missing", 0, "", ""));
            continue;
        }
        let dst = bundle.join(safe_name(&rel));
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(src, &dst)?;
        let bytes = std::fs::read(src)?;
        rows.push(row(
            &rel,
            "present",
            bytes.len() as u64,
            &sha256(src),
            blake3::hash(&bytes).to_hex().as_ref(),
        ));
    }
    Ok(rows)
}

fn artifacts() -> Vec<String> {
    [
        "target/x86_64-nonos/release/nonos-kernel",
        "target/kernel_signed.bin",
        "target/kernel_attested.bin",
        "nonos-bootloader/target/x86_64-unknown-uefi/release/nonos_boot.efi",
        "target/esp/EFI/Boot/BOOTX64.EFI",
        "target/esp/EFI/nonos/kernel.bin",
        "target/esp/EFI/nonos/boot.cfg",
        "target/esp/startup.nsh",
        "nonos-data/trust/MANIFEST.sha256",
        "nonos-data/trust/policy/nonos_trust_anchor.policy.bin",
        "nonos-data/trust/policy/zk_capsule_policy_root.bin",
        "nonos-data/trust/zk/device_root.bin",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

fn row(path: &str, status: &str, bytes: u64, sha256: &str, b3: &str) -> serde_json::Value {
    serde_json::json!({ "path": path, "status": status, "bytes": bytes, "sha256": sha256, "blake3": b3 })
}

fn safe_name(rel: &str) -> PathBuf {
    rel.split('/').collect()
}

fn sha256(path: &Path) -> String {
    capture("shasum", &["-a", "256", path.to_str().unwrap_or("")])
        .1
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

fn sums(rows: &[serde_json::Value], field: &str) -> String {
    let mut out = String::new();
    for row in rows {
        if row["status"] == "present" {
            out.push_str(row[field].as_str().unwrap_or(""));
            out.push_str("  ");
            out.push_str(row["path"].as_str().unwrap_or(""));
            out.push('\n');
        }
    }
    out
}

fn provenance(artifacts: &[serde_json::Value]) -> serde_json::Value {
    serde_json::json!({
        "schema": "nonos.release.provenance.v1",
        "commit": capture_stdout("git", &["rev-parse", "HEAD"]).1.trim(),
        "ref": std::env::var("GITHUB_REF").unwrap_or_else(|_| "local".to_string()),
        "run_id": std::env::var("GITHUB_RUN_ID").unwrap_or_else(|_| "local".to_string()),
        "source_date_epoch": std::env::var("SOURCE_DATE_EPOCH").unwrap_or_else(|_| "unset".to_string()),
        "toolchain": std::fs::read_to_string("rust-toolchain.toml").unwrap_or_default(),
        "artifacts": artifacts,
    })
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
