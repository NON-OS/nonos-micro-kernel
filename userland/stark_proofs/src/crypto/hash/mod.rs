// NONOS Operating System (AGPL-3.0-or-later)
#[allow(
    clippy::needless_range_loop,
    clippy::manual_rotate,
    clippy::unnecessary_cast,
    clippy::manual_is_multiple_of
)]
#[path = "../../../../../src/crypto/hash/blake3/mod.rs"]
pub mod blake3;
pub use blake3::blake3_hash;

#[allow(clippy::all)]
#[path = "../../../../../src/crypto/hash/sha3/mod.rs"]
pub mod sha3;
pub use sha3::keccak256;
