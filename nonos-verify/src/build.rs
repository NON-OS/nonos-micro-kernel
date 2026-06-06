// build: compile proof. Formatting, clippy, the production desktop GUI profile,
// section sizes, and a binary symbol scan on the built release ELF. x86_64 is
// blocking; other arches are gaps until a kernel build lane exists.

use crate::report::{Report, Status};
use crate::sh::{capture, have, run_logged};
use std::path::Path;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("build", true);
    let out = Path::new(root).join("build");
    std::fs::create_dir_all(&out)?;

    let ok = run_logged("cargo", &["fmt", "--all", "--", "--check"], &out.join("rustfmt.txt"));
    rpt.check("rustfmt", st(ok), "cargo fmt --all --check");

    let ok = run_logged(
        "cargo",
        &["clippy", "--manifest-path", "nonos-sign/Cargo.toml", "--all-targets", "--", "-D", "warnings"],
        &out.join("clippy.txt"),
    );
    rpt.check("clippy-nonos-sign", st(ok), "clippy -D warnings (nonos-sign)");

    let ok = run_logged("make", &["nonos-mk-desktop-gui-prod"], &out.join("build-x86_64.txt"));
    rpt.check("build-x86_64-desktop-gui-prod", st(ok), "make nonos-mk-desktop-gui-prod");

    let kbin = "target/x86_64-nonos/release/nonos-kernel";
    if Path::new(kbin).exists() {
        let (_, sz) = capture("size", &[kbin]);
        let (_, se) = capture("readelf", &["-S", kbin]);
        std::fs::write(out.join("section-size-report.txt"), format!("{sz}\n---\n{se}"))?;
        let _ = std::fs::copy(kbin, out.join("nonos-kernel.x86_64"));
        rpt.check("section-size", Status::Pass, "size + readelf -S captured on release ELF");

        let scan = out.join("symbol-scan.txt");
        if Path::new("nonos-ci/scan-microkernel-symbols.sh").exists() {
            let ok = run_logged("bash", &["nonos-ci/scan-microkernel-symbols.sh"], &scan);
            rpt.check("symbol-scan", st(ok), "binary symbol scan (nonos-ci/scan-microkernel-symbols.sh)");
        } else {
            let nm = if have("llvm-nm") { "llvm-nm" } else { "nm" };
            let (_, syms) = capture(nm, &["--demangle", kbin]);
            let hits: Vec<&str> = syms
                .lines()
                .filter(|l| l.contains("core::panicking") || l.contains("rust_begin_unwind"))
                .collect();
            std::fs::write(&scan, hits.join("\n"))?;
            rpt.check(
                "symbol-scan",
                if hits.is_empty() { Status::Pass } else { Status::Fail },
                "panic-machinery scan on release ELF",
            );
        }
    } else {
        rpt.check("section-size", Status::Skip, "kernel ELF absent (build did not produce it)");
    }

    for arch in ["aarch64", "riscv64"] {
        if Path::new(&format!("{arch}-nonos.json")).exists() {
            rpt.check(&format!("build-{arch}"), Status::Gap, "kernel target json present, build lane not wired");
        } else {
            rpt.gap(
                format!("kernel build lane for {arch}"),
                format!("a {arch}-nonos.json kernel target + a make build target (only userland/{arch}-nonos-user.json exists today)"),
            );
        }
    }

    rpt.finish(root)
}

fn st(ok: bool) -> Status {
    if ok {
        Status::Pass
    } else {
        Status::Fail
    }
}
