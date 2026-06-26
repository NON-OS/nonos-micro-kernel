// attest: fuse every module report into one ci-attestation.json + a GitHub job
// summary. Fails if any blocking module failed. Facts only.

use crate::report::{Report, Status};
use std::fmt::Write as _;
use std::path::Path;

pub fn run(root: &str) -> std::io::Result<Status> {
    let root_path = Path::new(root);
    let meta = Report::new("attest", true); // for commit + toolchain
    let out = root_path.join("attest");
    std::fs::create_dir_all(&out)?;

    let mut modules: Vec<serde_json::Value> = Vec::new();
    let mut gaps: Vec<serde_json::Value> = Vec::new();
    let mut blocking_fail = false;
    let required = required_modules();

    if let Ok(rd) = std::fs::read_dir(root_path) {
        let mut dirs: Vec<_> =
            rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.is_dir()).collect();
        dirs.sort();
        for d in dirs {
            let name = d.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            if name == "attest" {
                continue;
            }
            let report = d.join(format!("{name}-report.json"));
            if !report.exists() {
                continue;
            }
            let bytes = std::fs::read(&report).unwrap_or_default();
            let hash = blake3::hash(&bytes).to_hex().to_string();
            let v: serde_json::Value =
                serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
            let status = v.get("status").and_then(|s| s.as_str()).unwrap_or("unknown");
            let blocking = v.get("blocking").and_then(|b| b.as_bool()).unwrap_or(true);
            if status == "fail" && blocking {
                blocking_fail = true;
            }
            if let Some(gs) = v.get("gaps").and_then(|g| g.as_array()) {
                for g in gs {
                    gaps.push(g.clone());
                }
            }
            modules.push(serde_json::json!({
                "module": name,
                "status": status,
                "blocking": blocking,
                "report_blake3": hash,
            }));
        }
    }

    let present: std::collections::BTreeSet<String> = modules
        .iter()
        .filter_map(|m| m.get("module").and_then(|n| n.as_str()).map(str::to_string))
        .collect();
    let missing: Vec<String> = required.iter().filter(|m| !present.contains(*m)).cloned().collect();
    if !missing.is_empty() {
        blocking_fail = true;
    }

    let attestation = serde_json::json!({
        "commit": meta.commit,
        "toolchain": meta.toolchain,
        "modules": modules,
        "required_modules": required,
        "missing_modules": missing,
        "gaps": gaps,
        "blocking_failure": blocking_fail,
    });
    std::fs::write(
        out.join("ci-attestation.json"),
        serde_json::to_string_pretty(&attestation).unwrap(),
    )?;

    let mut md = String::new();
    let _ = writeln!(md, "# NONOS verification attestation\n");
    let _ = writeln!(md, "- commit: `{}`", meta.commit);
    let _ = writeln!(md, "- toolchain: `{}`\n", meta.toolchain);
    let _ = writeln!(md, "| module | status | blocking |\n|---|---|---|");
    for m in &modules {
        let _ = writeln!(
            md,
            "| {} | {} | {} |",
            m["module"].as_str().unwrap_or(""),
            m["status"].as_str().unwrap_or(""),
            m["blocking"].as_bool().unwrap_or(true)
        );
    }
    if !missing.is_empty() {
        let _ = writeln!(md, "\nmissing required modules: `{}`", missing.join(","));
    }
    let _ = writeln!(md, "\nknown gaps: {}", gaps.len());
    let _ = writeln!(md, "\nblocking failure: {blocking_fail}");
    std::fs::write(out.join("summary.md"), &md)?;

    if let Ok(p) = std::env::var("GITHUB_STEP_SUMMARY") {
        use std::io::Write as _;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
            let _ = f.write_all(md.as_bytes());
        }
    }

    if blocking_fail {
        eprintln!("attest: a blocking module failed");
        Ok(Status::Fail)
    } else {
        eprintln!("attest: no blocking failures ({} modules)", modules.len());
        Ok(Status::Pass)
    }
}

fn required_modules() -> Vec<String> {
    std::env::var("NONOS_VERIFY_REQUIRED_MODULES")
        .unwrap_or_else(|_| "build,supply-chain,trust-chain,adversarial,evidence".to_string())
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}
