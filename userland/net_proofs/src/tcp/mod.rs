// NONOS Operating System (AGPL-3.0-or-later)
#[allow(dead_code, clippy::all)]
#[path = "../../../capsule_net_tcp/src/tcp/checksum.rs"]
mod checksum;
#[allow(dead_code, clippy::all)]
#[path = "../../../capsule_net_tcp/src/tcp/header.rs"]
pub mod header;
#[allow(dead_code, clippy::all)]
#[path = "../../../capsule_net_tcp/src/tcp/parse.rs"]
pub mod parse;
#[allow(dead_code, clippy::all)]
#[path = "../../../capsule_net_tcp/src/tcp/seq.rs"]
pub mod seq;

// Matches the capsule constant (tcp/mod.rs).
pub const REASM_MAX_SEGS: usize = 32;
