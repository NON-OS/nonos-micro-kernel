// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the STARK verification primitives. Includes the
//! real src/crypto/stark source and checks it against its specification.

#[path = "../../../src/crypto/stark/mod.rs"]
pub mod stark;

#[cfg(test)]
mod field_tests;
