// NONOS Operating System (AGPL-3.0-or-later)
// The real RX/TX queue types, the used-ring readers, and the avail-ring
// refill, exactly as the driver ships them.
#[path = "../../../capsule_driver_virtio_net/src/queue/post.rs"]
mod post;
#[path = "../../../capsule_driver_virtio_net/src/queue/rx_queue.rs"]
mod rx_queue;
#[path = "../../../capsule_driver_virtio_net/src/queue/tx_queue.rs"]
mod tx_queue;
// Upstream buffer_mut is unsafe without a # Safety section and returns a
// mutable slice from &self; kept as it ships.
#[path = "../../../capsule_driver_virtio_net/src/queue/used.rs"]
#[allow(clippy::missing_safety_doc)]
#[allow(clippy::mut_from_ref)]
mod used;

pub use rx_queue::RxQueue;
pub use tx_queue::TxQueue;
