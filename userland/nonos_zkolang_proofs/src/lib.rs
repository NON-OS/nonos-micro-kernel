// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the zkolang VM. A real VM run is laid out for the
//! ALU step AIR and driven through the money-grade Poseidon-committed STARK, so
//! the honest trace is proven to accept and each tampered trace to be rejected.

#[cfg(test)]
mod commit_tests;
#[cfg(test)]
mod feature_tests;
#[cfg(test)]
mod fn_tests;
#[cfg(test)]
mod io_tests;
#[cfg(test)]
mod lang_tests;
#[cfg(test)]
mod loop_tests;
#[cfg(test)]
mod nox_tests;
#[cfg(test)]
mod operator_tests;
#[cfg(test)]
mod recipes_tests;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
mod vkey_tests;
#[cfg(test)]
mod witness_tests;
