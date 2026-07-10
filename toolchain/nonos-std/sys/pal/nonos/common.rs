// NONOS std PAL runtime hooks. The main thread's TLS table installs
// itself lazily on first access (rt::init reads the current thread id
// before this init hook runs, so an eager claim here would come too
// late, and a second claim would discard that first table). Cleanup
// runs the main thread's TLS destructors after main returns. The
// unsupported-platform error helpers are the same shape every minimal
// PAL carries.

use crate::io as std_io;

// SAFETY: must be called only once during runtime initialization.
// NOTE: this is not guaranteed to run, for example when Rust code is called externally.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {
    // SAFETY: init_tls ran in `init`, and no thread-local access follows
    // runtime cleanup on the main thread.
    unsafe { crate::sys::thread_local::key::destroy_tls() };
}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::UNSUPPORTED_PLATFORM
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}
