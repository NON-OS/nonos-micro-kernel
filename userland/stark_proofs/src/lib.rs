// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the STARK verification primitives. Includes the
//! real src/crypto source and checks it against its specification.

extern crate alloc;

pub mod crypto;

#[cfg(test)]
mod air_tests;
#[cfg(test)]
mod field_ext_tests;
#[cfg(test)]
mod field_tests;
#[cfg(test)]
mod forgery_tests;
#[cfg(test)]
mod fri_ext_tests;
#[cfg(test)]
mod fri_poseidon_ext_tests;
#[cfg(test)]
mod fri_poseidon_tests;
#[cfg(test)]
mod fri_tests;
#[cfg(test)]
mod merkle_tests;
#[cfg(test)]
mod ntt_tests;
#[cfg(test)]
mod poly_tests;
#[cfg(test)]
mod stark_selftest_gen;
#[cfg(test)]
mod poseidon_constants_gen;
#[cfg(test)]
mod poseidon_merkle_tests;
