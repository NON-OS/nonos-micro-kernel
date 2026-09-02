// supply-chain: advisories, license/ban policy, duplicate surface, submodule pin
// integrity, SBOM, reproducibility. Real tools invoked from Rust; missing
// tooling and the heavy repro lane become explicit gaps.

use crate::report::{Report, Status};
use crate::sh::{capture, capture_stdout, have, run_logged};
use std::path::Path;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("supply-chain", true);
    let out = Path::new(root).join("supply-chain");
    std::fs::create_dir_all(&out)?;

    // advisories
    if have("cargo-audit") {
        let (ok, s) = capture("cargo", &["audit", "--json"]);
        std::fs::write(out.join("advisory-report.json"), &s)?;
        rpt.check("cargo-audit", st(ok), "cargo audit");
    } else {
        rpt.check("cargo-audit", Status::Gap, "cargo-audit not installed");
        rpt.gap("advisory scan", "nonos-setup must install cargo-audit (tools: audit)");
    }

    // license + ban policy
    if have("cargo-deny") {
        let ok = run_logged("cargo", &["deny", "check"], &out.join("cargo-deny.txt"));
        rpt.check("cargo-deny", st(ok), "cargo deny check against deny.toml");
    } else {
        rpt.check("cargo-deny", Status::Gap, "cargo-deny not installed");
        rpt.gap("license/ban policy", "nonos-setup must install cargo-deny (tools: deny)");
    }

    // duplicate / high-risk dependency surface
    run_logged("cargo", &["tree", "--workspace", "--duplicates"], &out.join("dep-duplicates.txt"));
    rpt.check("dep-duplicates", Status::Pass, "duplicate dependency report captured");

    // submodule pin integrity
    let (_, status) = capture_stdout("git", &["submodule", "status", "--recursive"]);
    let mut bad = false;
    let mut subs: Vec<serde_json::Value> = Vec::new();
    for line in status.lines() {
        if line.is_empty() {
            continue;
        }
        let flag = line.chars().next().unwrap_or(' ');
        let state = match flag {
            '+' => "ahead-of-pin",
            '-' => "uninitialized",
            'U' => "merge-conflict",
            _ => "clean",
        };
        if matches!(flag, '+' | '-' | 'U') {
            bad = true;
        }
        let mut it = line[1..].split_whitespace();
        let sha = it.next().unwrap_or("");
        let path = it.next().unwrap_or("");
        subs.push(serde_json::json!({ "path": path, "sha": sha, "state": state }));
    }
    std::fs::write(
        out.join("submodule-report.json"),
        serde_json::to_string_pretty(&serde_json::json!({ "submodules": subs })).unwrap(),
    )?;
    rpt.check(
        "submodule-pins",
        if bad { Status::Fail } else { Status::Pass },
        "submodules on committed pins",
    );

    // SBOM
    if have("cargo-cyclonedx") {
        let ok = run_logged("cargo", &["cyclonedx", "--format", "json"], &out.join("sbom.log"));
        rpt.check("sbom", st(ok), "CycloneDX SBOM");
    } else {
        rpt.check("sbom", Status::Fail, "cargo-cyclonedx not installed");
    }

    // reproducibility: the double-build hash compare runs in the nightly lane.
    std::fs::write(
        out.join("reproducibility-report.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "source_date_epoch": std::env::var("SOURCE_DATE_EPOCH").unwrap_or_else(|_| "unset".into()),
            "note": "bit-for-bit double-build comparison runs in the nightly lane",
        }))
        .unwrap(),
    )?;
    rpt.gap(
        "bit-for-bit reproducibility gate",
        "nightly: build twice with fixed SOURCE_DATE_EPOCH and compare blake3 of kernel + capsule artifacts",
    );

    rpt.finish(root)
}

fn st(ok: bool) -> Status {
    if ok {
        Status::Pass
    } else {
        Status::Fail
    }
}
