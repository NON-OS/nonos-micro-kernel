// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the STARK verification primitives. Includes the
//! real src/crypto source and checks it against its specification.

extern crate alloc;

pub mod crypto;

#[cfg(test)]
mod field_tests;
#[cfg(test)]
mod fri_tests;
#[cfg(test)]
mod merkle_tests;
#[cfg(test)]
mod poly_tests;
