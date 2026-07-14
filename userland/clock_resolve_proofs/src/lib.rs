// NONOS Operating System (AGPL-3.0-or-later)
//! Host proof for the boot wall-clock source picker. The real picker is included
//! via #[path]; a thin public wrapper keeps it referenced in the library build
//! while the assertions live in the tests.

#[path = "../../../src/sys/clock/resolve.rs"]
mod resolve;

/// Non-test reference so the included picker is not flagged unused in the
/// library build.
pub fn pick(handoff: u64, calibrated: u64, fresh: u64) -> u64 {
    resolve::pick_nonzero(handoff, || calibrated, || fresh)
}

#[cfg(test)]
mod tests;
