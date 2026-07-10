// NONOS Operating System (AGPL-3.0-or-later)
// The real NVMe read/write request parser (pub(super); wrapped for the proofs).
#[path = "../../../../capsule_driver_nvme/src/server/handlers/rw_parse.rs"]
mod rw_parse;

use crate::nvm::MAX_SECTORS;

// The parser now bounds the request by the drive's per-command sector ceiling
// (LBA-size aware). The proofs model the 512-byte default, whose ceiling is
// MAX_SECTORS, so the harness pins that same bound the tests assert against.
pub fn parse_rw(body: &[u8], capacity: u64) -> Result<(u64, u32), i32> {
    rw_parse::parse(body, capacity, MAX_SECTORS)
}
