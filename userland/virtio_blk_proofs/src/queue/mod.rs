// NONOS Operating System (AGPL-3.0-or-later)
// The real virtqueue layout type. Its setup constructor is never called on
// the host; the proofs build the struct literally with null pointers.
#[path = "../../../capsule_driver_virtio_blk/src/queue/layout.rs"]
mod layout;

pub use layout::Queue;
