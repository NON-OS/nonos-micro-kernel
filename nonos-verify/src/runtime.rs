// runtime: bounded QEMU boot proof. The verifier runs the existing Plan-A
// harness, keeps the serial/result logs as artifacts, and turns boot markers
// into a typed CI decision.

use crate::report::{Report, Status};
use std::path::Path;
use std::process::Command;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("runtime", true);
    let out = Path::new(root).join("runtime");
    std::fs::create_dir_all(&out)?;

    let serial = out.join("qemu-serial.log");
    let result = out.join("plan-a-result.log");
    let output = Command::new("make")
        .arg("nonos-mk-plan-a-runtime")
        .env("PLAN_A_SERIAL", &serial)
        .env("PLAN_A_RESULT", &result)
        .output();

    let mut boot_ok = false;
    match output {
        Ok(o) => {
            let mut buf = Vec::with_capacity(o.stdout.len() + o.stderr.len());
            buf.extend_from_slice(&o.stdout);
            buf.extend_from_slice(&o.stderr);
            std::fs::write(out.join("runtime-run.log"), &buf)?;
            boot_ok = o.status.success();
        }
        Err(e) => {
            std::fs::write(out.join("runtime-run.log"), format!("failed to spawn make: {e}\n"))?;
        }
    }
    rpt.check("plan-a-harness", st(boot_ok), "make nonos-mk-plan-a-runtime");

    let serial_text = std::fs::read_to_string(&serial).unwrap_or_default();
    let result_text = std::fs::read_to_string(&result).unwrap_or_default();
    let joined = format!("{serial_text}\n{result_text}");

    let no_fatal = !joined.contains("[FATAL]")
        && !joined.contains("[PANIC]")
        && !joined.contains("KERNEL PANIC")
        && !joined.contains("Hardware requirements not met");
    rpt.check("no-fatal-marker", st(no_fatal), "serial log has no fatal/panic marker");

    let no_zk_fail = !joined.contains("[ZK-ATTEST] FAIL")
        && !joined.contains("ZK proof missing")
        && !joined.contains("attestation failed");
    rpt.check("zk-attestation-not-failed", st(no_zk_fail), "serial log has no ZK rejection marker");

    let handoff = joined.contains("[NONOS] Handoff OK") || joined.contains("Handoff OK");
    rpt.check("boot-handoff", st(handoff), "boot handoff marker observed");

    let capsules =
        joined.contains("[INIT] Capsules spawned") || joined.contains("Capsules spawned");
    rpt.check("capsules-spawned", st(capsules), "capsule spawn readiness marker observed");

    // The kernel's boot log prints these uppercase; the lowercase forms are
    // kept for older serial logs so the check reads both eras.
    let desktop = joined.contains("[COMPOSITOR] capsule spawned")
        || joined.contains("[WM] capsule spawned")
        || joined.contains("name=desktop_shell")
        || joined.contains("[desktop_shell]")
        || joined.contains("[wm] boot")
        || joined.contains("[compositor]");
    rpt.check("desktop-substrate", st(desktop), "desktop/compositor marker observed");

    rpt.finish(root)
}

fn st(ok: bool) -> Status {
    if ok {
        Status::Pass
    } else {
        Status::Fail
    }
}
