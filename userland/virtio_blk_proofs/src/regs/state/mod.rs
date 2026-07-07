// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../../capsule_driver_virtio_blk/src/regs/state/mmio.rs"]
mod mmio;
// The io field is only read by the MMIO/PIO accessors, which the parsers
// never use and the proofs do not include.
#[path = "../../../../capsule_driver_virtio_blk/src/regs/state/types.rs"]
#[allow(dead_code)]
mod types;

pub use types::Regs;
