// NONOS Operating System (AGPL-3.0-or-later)
//! Binary Merkle commitment over field-element leaves, using the proven BLAKE3.
//! The prover commits and opens; the verifier calls `verify_path`.

mod hash;
mod tree;
mod verify;

pub use hash::hash_leaf_wide_periodic;
pub use tree::MerkleTree;
pub use verify::{verify_path, verify_path_ext, verify_path_wide, verify_path_wide_periodic};
