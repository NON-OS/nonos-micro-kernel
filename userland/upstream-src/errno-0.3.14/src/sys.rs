//! A default sys.rs for unrecognized targets.
//!
//! If lib.rs doesn't recognize the target, it defaults to using this file. On
//! NONOS it provides a real thread-local errno; on every other unrecognized
//! target it still issues the explanatory compile error.

// If there is no OS, there's no `errno` or equivalent defined. NONOS is such a
// target but supplies its own compatibility errno below, so it is excluded here.
#[cfg(all(any(target_os = "unknown", target_os = "none"), not(target_vendor = "nonos")))]
compile_error!("The target OS is \"unknown\" or \"none\", so it's unsupported by the errno crate.");

// If there is an OS, support may be added.
#[cfg(all(not(any(target_os = "unknown", target_os = "none")), not(target_vendor = "nonos")))]
compile_error!("The target OS is not yet supported in the errno crate.");

use crate::Errno;

// NONOS capsules report syscall failure through Result values, so there is no
// kernel errno location. This keeps one process-wide slot for the crate's API
// using only core (the crate is no_std): set_errno stores, errno reads it back,
// and descriptions fall through to the raw code since there is no strerror table.
#[cfg(target_vendor = "nonos")]
static NONOS_ERRNO: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(0);

pub fn with_description<F, T>(err: Errno, callback: F) -> T
where
    F: FnOnce(Result<&str, Errno>) -> T,
{
    callback(Err(err))
}

pub const STRERROR_NAME: &str = "";

pub fn errno() -> Errno {
    Errno(NONOS_ERRNO.load(core::sync::atomic::Ordering::Relaxed))
}

pub fn set_errno(err: Errno) {
    NONOS_ERRNO.store(err.0, core::sync::atomic::Ordering::Relaxed);
}
