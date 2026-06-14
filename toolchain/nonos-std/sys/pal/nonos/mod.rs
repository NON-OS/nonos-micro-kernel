// NONOS std PAL bootstrap. Reuses the unsupported os and common layers
// (init, cleanup, abort, the os stubs) and provides a real time clock.
#![deny(unsafe_op_in_unsafe_fn)]

pub mod os;
pub mod time;

#[path = "../unsupported/common.rs"]
mod common;
pub use common::*;
