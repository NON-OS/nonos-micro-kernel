// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the STARK verification primitives. Includes the
//! real src/crypto source and checks it against its specification.

extern crate alloc;

pub mod crypto;

#[cfg(test)]
mod aggregate_tests;
#[cfg(test)]
mod air_tests;
#[cfg(test)]
mod barycentric_tests;
#[cfg(test)]
mod enroll_batch_tests;
#[cfg(test)]
mod field_ext_tests;
#[cfg(test)]
mod field_tests;
#[cfg(test)]
mod fold_witness_tests;
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
mod parallel_commit_tests;
#[cfg(test)]
mod parallel_prover_gate;
#[cfg(test)]
mod periodic_z_tests;
#[cfg(test)]
mod poly_tests;
#[cfg(test)]
mod poseidon_constants_gen;
#[cfg(test)]
mod poseidon_merkle_tests;
#[cfg(test)]
mod poseidon_pub_tests;
#[cfg(test)]
mod preprocessed_tests;
#[cfg(test)]
mod production_vector_gen;
#[cfg(test)]
mod recursion_assembly;
#[cfg(test)]
mod recursion_assembly_tests;
#[cfg(test)]
mod shared_root_tests;
#[cfg(test)]
mod stark_selftest_gen;

// Machine-checked proof harnesses, compiled only under `cargo kani`.
#[cfg(kani)]
mod kani_proofs;
