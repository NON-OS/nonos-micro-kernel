// NONOS Operating System (AGPL-3.0-or-later)

/// Host stub for the uptime syscall. The tested constructors and comparison
/// (`at`, `is_past`) never call it; only the live `after_ms`/`expired` paths do.
pub fn mk_uptime_ms() -> i64 {
    0
}

#[path = "../../../libc/src/time/deadline.rs"]
mod deadline;

pub use deadline::Deadline;
