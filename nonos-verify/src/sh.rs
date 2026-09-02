// Thin, typed helpers for invoking external tools (make, cargo, qemu, nm).
// Invoking a process is legitimate; the decisions and reporting stay in Rust.

use std::path::Path;
use std::process::Command;

/// Run a command, write combined stdout+stderr to `log`, return success.
pub fn run_logged(program: &str, args: &[&str], log: &Path) -> bool {
    match Command::new(program).args(args).output() {
        Ok(o) => {
            let mut buf = Vec::with_capacity(o.stdout.len() + o.stderr.len());
            buf.extend_from_slice(&o.stdout);
            buf.extend_from_slice(&o.stderr);
            let _ = std::fs::write(log, &buf);
            o.status.success()
        }
        Err(e) => {
            let _ = std::fs::write(log, format!("failed to spawn {program}: {e}\n"));
            false
        }
    }
}

/// Run a command and capture stdout alone. For output that gets parsed;
/// git prints advisory warnings on stderr and a combined capture turns
/// them into phantom data.
pub fn capture_stdout(program: &str, args: &[&str]) -> (bool, String) {
    match Command::new(program).args(args).output() {
        Ok(o) => (o.status.success(), String::from_utf8_lossy(&o.stdout).into_owned()),
        Err(e) => (false, format!("failed to spawn {program}: {e}")),
    }
}

/// Run a command and capture combined output plus success.
pub fn capture(program: &str, args: &[&str]) -> (bool, String) {
    match Command::new(program).args(args).output() {
        Ok(o) => {
            let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
            s.push_str(&String::from_utf8_lossy(&o.stderr));
            (o.status.success(), s)
        }
        Err(e) => (false, format!("failed to spawn {program}: {e}")),
    }
}

/// Is a tool on PATH?
pub fn have(tool: &str) -> bool {
    capture("sh", &["-c", &format!("command -v {tool}")]).0
}
