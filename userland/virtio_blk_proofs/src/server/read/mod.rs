// NONOS Operating System (AGPL-3.0-or-later)
// The real read-request parser (pub(super); wrapped for the proofs).
#[path = "../../../../capsule_driver_virtio_blk/src/server/handlers/read/request.rs"]
mod request;

use crate::protocol::Request;
use crate::setup::Driver;

pub fn parse_read(driver: &Driver, req: &Request, body: &[u8]) -> Result<(u64, u32, usize), i32> {
    request::read_request(driver, req, body).map(|r| (r.lba, r.nsectors, r.bytes_n))
}
