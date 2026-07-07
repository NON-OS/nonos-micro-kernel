// NONOS Operating System (AGPL-3.0-or-later)
// The constant-time primitives (BLAKE3 dependency), the hash, and the stark
// module, mirrored at their kernel paths so the included source resolves.
#[allow(clippy::manual_memcpy, clippy::needless_range_loop, clippy::manual_rotate)]
#[path = "../../../../src/crypto/util/constant_time/mod.rs"]
pub mod constant_time;

pub mod hash;

#[path = "../../../../src/crypto/stark/mod.rs"]
pub mod stark;
