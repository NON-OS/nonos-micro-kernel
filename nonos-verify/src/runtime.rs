// runtime: proof in QEMU. Runs the real boot harnesses and records which
// readiness/PASS markers each emitted. No path is claimed to work unless its
// harness exists and ran. Input end-to-end is an explicit gap.

use crate::report::{Report, Status};
use crate::sh::run_logged;
use std::path::Path;

const LANES: &[(&str, &str, &str, &str)] = &[
    // (make target, name, required marker, timeout seconds). The first lane pays
    // the cold compile of the ZK proving toolchain (ark-groth16, the bootloader
    // embed tools) before QEMU even starts, so the budgets cover compile + boot.
    ("nonos-mk-boot-ramfs", "ramfs", "PASS", "900"),
    ("nonos-mk-boot-keyring", "keyring", "PASS", "600"),
    ("nonos-mk-boot-desktop-gui", "desktop-gui", "PASS", "900"),
];

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("runtime", true);
    let out = Path::new(root).join("runtime");
    let traces = out.join("boot-traces");
    std::fs::create_dir_all(&traces)?;

    for (target, name, marker, tmo) in LANES {
        let log = traces.join(format!("{name}.log"));
        // `timeout <s> make <target>` so a hung QEMU cannot stall the lane.
        let ok = run_logged("timeout", &[tmo, "make", target], &log);
        let content = std::fs::read_to_string(&log).unwrap_or_default();
        let observed = content.contains(marker);
        if observed {
            rpt.check(&format!("boot-{name}"), Status::Pass, format!("marker '{marker}' observed"));
        } else if !ok {
            rpt.check(
                &format!("boot-{name}"),
                Status::Fail,
                format!("harness failed and '{marker}' absent"),
            );
        } else {
            rpt.check(
                &format!("boot-{name}"),
                Status::Fail,
                format!("marker '{marker}' not observed"),
            );
        }
    }

    // Subsystem round-trips that already have scripts. Each embeds its capsule's
    // signed id-cert and manifest at compile time, so the per-capsule trust
    // artifacts must be provisioned first. When they are absent this lane cannot
    // build the test kernel; that is a provisioning gap, not a runtime failure.
    let capsule_trust = Path::new("nonos-data/trust/capsules");
    let provisioned = capsule_trust
        .read_dir()
        .map(|mut d| {
            d.any(|e| e.map(|e| e.path().extension().is_some_and(|x| x == "bin")).unwrap_or(false))
        })
        .unwrap_or(false);
    for s in [
        "entropy_round_trip",
        "crypto_hash_round_trip",
        "vfs_round_trip",
        "ps2_input_round_trip",
        "virtio_blk_round_trip",
        "market_round_trip",
    ] {
        let script = format!("tests/boot/{s}.sh");
        if !Path::new(&script).exists() {
            rpt.check(&format!("rt-{s}"), Status::Skip, "round-trip script absent");
        } else if !provisioned {
            rpt.check(
                &format!("rt-{s}"),
                Status::Skip,
                "signed capsule trust artifacts not provisioned in this lane",
            );
        } else {
            let log = traces.join(format!("{s}.log"));
            let ok = run_logged("timeout", &["300", "bash", &script], &log);
            rpt.check(
                &format!("rt-{s}"),
                if ok { Status::Pass } else { Status::Fail },
                "round-trip script",
            );
        }
    }

    rpt.gap(
        "input end-to-end proof",
        "a QEMU harness that injects a synthetic key/pointer event and asserts it reaches the desktop shell",
    );

    rpt.finish(root)
}
