// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../capsule_net_tcp/src/tcp/checksum.rs"]
pub mod checksum;
#[path = "../../../capsule_net_tcp/src/tcp/header.rs"]
pub mod header;
#[path = "../../../capsule_net_tcp/src/tcp/parse.rs"]
pub mod parse;
#[path = "../../../capsule_net_tcp/src/tcp/seq.rs"]
pub mod seq;

// Matches the capsule constant (tcp/mod.rs).
pub const REASM_MAX_SEGS: usize = 32;
