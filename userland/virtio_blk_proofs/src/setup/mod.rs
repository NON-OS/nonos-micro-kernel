// NONOS Operating System (AGPL-3.0-or-later)
// The real driver state struct the request parsers receive.
#[path = "../../../capsule_driver_virtio_blk/src/setup/driver.rs"]
mod driver;

pub use driver::Driver;
