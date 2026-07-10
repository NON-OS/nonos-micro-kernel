// NONOS std PAL bootstrap: a real time clock, a polling futex, and
// runtime hooks that own the main thread's TLS lifetime. The os stubs
// stay minimal until the kernel exposes an environment surface.
#![deny(unsafe_op_in_unsafe_fn)]

pub mod futex;
pub mod os;
pub mod time;

mod common;
pub use common::*;
