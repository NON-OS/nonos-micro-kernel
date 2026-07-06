// NONOS Operating System (AGPL-3.0-or-later)
// The real register-access type, without the MMIO/PIO accessors the parsers
// never touch. The PIO variant is unused on the host.
#[path = "../../../capsule_driver_virtio_blk/src/regs/io.rs"]
#[allow(dead_code)]
mod io;
mod state;

pub use state::Regs;
