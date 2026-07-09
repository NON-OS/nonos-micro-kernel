// NONOS Operating System (AGPL-3.0-or-later)
// The real AHCI read/write request parser (pub(super); wrapped for the proofs).
#[path = "../../../../capsule_driver_ahci/src/server/handlers/rw_parse.rs"]
mod rw_parse;

pub fn parse_rw(body: &[u8], capacity: u64) -> Result<(u64, u32), i32> {
    rw_parse::parse(body, capacity)
}
