// NONOS Operating System (AGPL-3.0-or-later)
// The constant-time primitives (BLAKE3 dependency), the hash, and the stark
// module, mirrored at their kernel paths so the included source resolves.
#[allow(clippy::manual_memcpy, clippy::needless_range_loop, clippy::manual_rotate)]
#[path = "../../../../src/crypto/util/constant_time/mod.rs"]
pub mod constant_time;

pub mod hash;

// The stark stack now lives in the nonos-stark crate. Re-export it here so the
// proofs run against the same source the kernel and bootloader link, addressed
// at the path the tests already use.
pub use nonos_stark as stark;
