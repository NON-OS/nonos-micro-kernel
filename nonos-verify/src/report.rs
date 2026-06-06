// Typed CI report. Each module builds one of these and writes a machine
// readable JSON plus a facts-only Markdown summary under ci-reports/<module>/.
// Status rolls up the checks: fail beats gap beats skip beats pass.

use serde::Serialize;
use std::fmt::Write as _;
use std::path::Path;
use std::process::Command;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pass,
    Fail,
    Skip,
    Gap,
}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Status::Pass => "pass",
            Status::Fail => "fail",
            Status::Skip => "skip",
            Status::Gap => "gap",
        }
    }
}

#[derive(Serialize)]
pub struct Check {
    pub name: String,
    pub status: Status,
    pub detail: String,
    pub artifact: String,
}

#[derive(Serialize)]
pub struct Gap {
    pub what: String,
    pub needs: String,
}

#[derive(Serialize)]
pub struct Report {
    pub module: String,
    pub commit: String,
    pub toolchain: String,
    pub status: Status,
    pub blocking: bool,
    pub checks: Vec<Check>,
    pub gaps: Vec<Gap>,
}

impl Report {
    pub fn new(module: &str, blocking: bool) -> Self {
        Report {
            module: module.to_string(),
            commit: commit(),
            toolchain: toolchain(),
            status: Status::Skip,
            blocking,
            checks: Vec::new(),
            gaps: Vec::new(),
        }
    }

    pub fn check(&mut self, name: &str, status: Status, detail: impl Into<String>) {
        self.checks.push(Check {
            name: name.to_string(),
            status,
            detail: detail.into(),
            artifact: String::new(),
        });
    }

    pub fn gap(&mut self, what: impl Into<String>, needs: impl Into<String>) {
        self.gaps.push(Gap {
            what: what.into(),
            needs: needs.into(),
        });
    }

    /// Roll up the overall status from the recorded checks and gaps.
    pub fn rollup(&mut self) -> Status {
        let any_fail = self.checks.iter().any(|c| c.status == Status::Fail);
        let any_gap = self.checks.iter().any(|c| c.status == Status::Gap) || !self.gaps.is_empty();
        let all_skip = !self.checks.is_empty() && self.checks.iter().all(|c| c.status == Status::Skip);
        self.status = if any_fail {
            Status::Fail
        } else if any_gap {
            Status::Gap
        } else if self.checks.is_empty() || all_skip {
            Status::Skip
        } else {
            Status::Pass
        };
        self.status
    }

    /// Write `<root>/<module>/<module>-report.json` + `-summary.md` (+ a
    /// `-gap.json` when gaps exist so a gap never passes silently).
    /// Returns the rolled-up status.
    pub fn finish(&mut self, root: &str) -> std::io::Result<Status> {
        let status = self.rollup();
        let dir = Path::new(root).join(&self.module);
        std::fs::create_dir_all(&dir)?;

        let json = serde_json::to_string_pretty(self).expect("report serializes");
        std::fs::write(dir.join(format!("{}-report.json", self.module)), json)?;

        let mut md = String::new();
        let _ = writeln!(md, "# {} report\n", self.module);
        let _ = writeln!(md, "| field | value |\n|---|---|");
        let _ = writeln!(md, "| module | {} |", self.module);
        let _ = writeln!(md, "| status | {} |", status.as_str());
        let _ = writeln!(md, "| blocking | {} |", self.blocking);
        let _ = writeln!(md, "| commit | {} |", self.commit);
        let _ = writeln!(md, "| toolchain | {} |\n", self.toolchain);
        let _ = writeln!(md, "## checks\n");
        if self.checks.is_empty() {
            let _ = writeln!(md, "_none_");
        } else {
            let _ = writeln!(md, "| name | status | detail | artifact |\n|---|---|---|---|");
            for c in &self.checks {
                let _ = writeln!(
                    md,
                    "| {} | {} | {} | {} |",
                    c.name,
                    c.status.as_str(),
                    c.detail.replace('|', "\\|"),
                    c.artifact
                );
            }
        }
        if !self.gaps.is_empty() {
            let _ = writeln!(md, "\n## gaps\n");
            let _ = writeln!(md, "| what | needs |\n|---|---|");
            for g in &self.gaps {
                let _ = writeln!(md, "| {} | {} |", g.what.replace('|', "\\|"), g.needs.replace('|', "\\|"));
            }
            let gj = serde_json::to_string_pretty(&serde_json::json!({
                "module": self.module,
                "gaps": self.gaps,
            }))
            .expect("gap json");
            std::fs::write(dir.join(format!("{}-gap.json", self.module)), gj)?;
        }
        std::fs::write(dir.join(format!("{}-summary.md", self.module)), md)?;
        Ok(status)
    }
}

fn commit() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn toolchain() -> String {
    std::fs::read_to_string("rust-toolchain.toml")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.trim_start().starts_with("channel"))
                .and_then(|l| l.split('"').nth(1).map(|c| c.to_string()))
        })
        .unwrap_or_else(|| "unknown".to_string())
}
