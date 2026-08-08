// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the page-table descriptor boundary.
//!
//! Both backends are included from the kernel tree by path, so these check the
//! shipping encoders rather than a copy, and both are compiled at once on the
//! host. That is the point: a boot only ever exercises one of them, and three
//! of the aarch64 rules are inverted or absent relative to x86_64.

#![allow(dead_code)]

pub mod arch;
pub mod civil;
pub mod descriptor;

#[cfg(test)]
mod civil_tests;
#[cfg(test)]
mod descriptor_tests;

#[cfg(kani)]
mod kani_proofs;
