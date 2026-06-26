// Evidence gate: host-side benchmark and bare-metal dossier tooling, machine
// metadata fail-closed checks, kernel source gate, and production bootloader
// public-anchor build. GitHub Actions only calls this typed verifier.

use crate::report::{Report, Status};
use crate::sh::run_logged;
use std::path::Path;
use std::process::Command;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("evidence", true);
    let out = Path::new(root).join("evidence");
    std::fs::create_dir_all(&out)?;
    let scripts = [
        "scripts/bare_metal_evidence.py",
        "scripts/validate_machine_metadata.py",
        "nonos-ci/bench_boot_log.py",
        "nonos-ci/bench_collect.py",
        "nonos-ci/bench_compare.py",
        "nonos-ci/bench_host.py",
        "nonos-ci/bench_suite.py",
        "nonos-ci/bench_time.py",
    ];
    let mut args = vec!["-m", "py_compile"];
    args.extend(scripts);
    let ok = run_logged("python3", &args, &out.join("python-syntax.txt"));
    rpt.check("python-syntax", st(ok), "benchmark and hardware evidence scripts compile");
    std::fs::create_dir_all("target/hardware-dossier")?;
    std::fs::write(
        "target/hardware-dossier/incomplete.json",
        r#"{"schema":"nonos.hardware.machine.v1","vendor":"fixture","model":"incomplete"}"#,
    )?;
    let bad = Command::new("make")
        .env("NONOS_HW_MACHINE_JSON", "target/hardware-dossier/incomplete.json")
        .arg("nonos-mk-validate-machine-metadata")
        .output()?;
    let mut bad_log = bad.stdout.clone();
    bad_log.extend_from_slice(&bad.stderr);
    std::fs::write(out.join("metadata-fail-closed.txt"), bad_log)?;
    rpt.check(
        "metadata-fail-closed",
        if bad.status.success() { Status::Fail } else { Status::Pass },
        "incomplete machine metadata is rejected",
    );
    std::fs::write(
        "target/hardware-dossier/full.json",
        r#"{"schema":"nonos.hardware.machine.v1","vendor":"fixture","model":"fixture","cpu":"fixture","firmware":"fixture","gpu":"fixture","display_path":"fixture","storage":"fixture","input":"fixture","iommu":"fixture","serial_capture":"fixture"}"#,
    )?;
    let good = Command::new("make")
        .env("NONOS_HW_MACHINE_JSON", "target/hardware-dossier/full.json")
        .arg("nonos-mk-validate-machine-metadata")
        .output()?;
    let mut good_log = good.stdout.clone();
    good_log.extend_from_slice(&good.stderr);
    std::fs::write(out.join("metadata-pass-fixture.txt"), good_log)?;
    rpt.check("metadata-pass-fixture", st(good.status.success()), "complete metadata accepted");
    let ok = run_logged("make", &["nonos-mk-check"], &out.join("kernel-source-check.txt"));
    rpt.check("kernel-source-check", st(ok), "make nonos-mk-check");
    std::fs::create_dir_all("target/ci")?;
    let pk = std::fs::read("nonos-data/trust/keys/nonos_trust_anchor_ed25519.pub")?;
    let pk_ok = pk.len() == 43 && &pk[..8] == b"NONOSPK1" && pk[10] == 32;
    if pk_ok {
        std::fs::write("target/ci/trust-anchor.raw", &pk[11..43])?;
    }
    let ok = pk_ok
        && Command::new("make")
            .env("BOOTLOADER_POLICY", "production")
            .env(
                "NONOS_TRUST_ANCHOR_PUBKEY",
                format!("{}/target/ci/trust-anchor.raw", std::env::current_dir()?.display()),
            )
            .arg("nonos-mk-bootloader")
            .output()
            .map(|o| {
                let mut log = o.stdout.clone();
                log.extend_from_slice(&o.stderr);
                let _ = std::fs::write(out.join("production-bootloader.txt"), log);
                o.status.success()
            })
            .unwrap_or(false);
    rpt.check(
        "production-bootloader-public-anchor",
        st(ok),
        "production bootloader builds from public trust anchor",
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
