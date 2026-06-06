// tests: run the host property + KAT suite (cargo test on this crate) and fold
// the result into the attestation. The test logic is the Rust in tests/; this
// drives it and reports pass/fail counts.

use crate::report::{Report, Status};
use crate::sh::capture;
use std::path::Path;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("tests", true);
    let out = Path::new(root).join("tests");
    std::fs::create_dir_all(&out)?;

    let (ok, output) =
        capture("cargo", &["test", "--manifest-path", "nonos-verify/Cargo.toml", "--tests"]);
    std::fs::write(out.join("cargo-test.txt"), &output)?;

    // Sum the libtest "test result: ok. N passed; M failed; ..." lines.
    let (mut passed, mut failed) = (0u32, 0u32);
    for line in output.lines() {
        if !line.starts_with("test result:") {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, w) in words.iter().enumerate() {
            if i == 0 {
                continue;
            }
            match *w {
                "passed;" | "passed" => passed += words[i - 1].parse().unwrap_or(0),
                "failed;" | "failed" => failed += words[i - 1].parse().unwrap_or(0),
                _ => {}
            }
        }
    }

    let status = if ok && failed == 0 { Status::Pass } else { Status::Fail };
    rpt.check("property-and-kat", status, format!("{passed} passed, {failed} failed"));
    rpt.finish(root)
}
