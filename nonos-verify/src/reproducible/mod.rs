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

//! Double-build reproducibility. Two detached worktrees of the same
//! commit build the same target; artifacts must then agree. Plain
//! artifacts agree byte for byte. NONOSIMG containers agree on their
//! measured kernel payload while the ML-DSA signature and the STARK
//! proof they carry are freshly randomized every run by design, so
//! those are required present and well formed, never identical.

mod compare;
mod lane;

use crate::report::{Report, Status};
use crate::sh::capture_stdout;
use compare::{artifacts, measure};
use std::path::Path;

pub fn run(root: &str) -> std::io::Result<Status> {
    let mut rpt = Report::new("reproducible", true);
    let out = Path::new(root).join("reproducible");
    let work = Path::new("target").join("repro-worktrees");
    let target = std::env::var("NONOS_REPRO_TARGET").unwrap_or_else(|_| "nonos-mk-esp".to_string());
    std::fs::create_dir_all(&out)?;
    std::fs::create_dir_all(&work)?;

    let dirt = capture_stdout("git", &["status", "--porcelain"]).1.trim().to_string();
    let clean =
        dirt.is_empty() || std::env::var("NONOS_REPRO_ALLOW_DIRTY").ok().as_deref() == Some("1");
    if clean {
        rpt.check("clean-tree", st(true), "git tree clean or explicitly allowed");
    } else {
        // Name the dirt in the report; a bare refusal costs a whole CI
        // round trip per offending path.
        std::fs::write(out.join("dirty-tree.txt"), &dirt)?;
        let first = dirt.lines().next().unwrap_or("");
        let detail = format!("tree dirty ({} paths, first: {first})", dirt.lines().count());
        rpt.check("clean-tree", st(false), detail);
        return rpt.finish(root);
    }

    let commit = capture_stdout("git", &["rev-parse", "HEAD"]).1.trim().to_string();
    let epoch = std::env::var("SOURCE_DATE_EPOCH")
        .unwrap_or_else(|_| capture_stdout("git", &["log", "-1", "--format=%ct"]).1.trim().to_string());
    lane::ensure_shared_keys(&out)?;
    let a_dir = lane::build("A", &work, &out, &commit, &epoch, &target)?;
    let b_dir = lane::build("B", &work, &out, &commit, &epoch, &target)?;
    let (verdicts, all_ok) = measure(&a_dir, &b_dir, artifacts());
    std::fs::write(out.join("verdicts.json"), serde_json::to_string_pretty(&verdicts).unwrap())?;
    lane::remove(&a_dir);
    lane::remove(&b_dir);
    rpt.check(
        "double-build-artifacts",
        st(all_ok),
        "payloads identical across two clean builds, randomized crypto present in both",
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
