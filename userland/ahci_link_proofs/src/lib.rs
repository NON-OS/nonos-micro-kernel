// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the AHCI link-up predicates. A directory tree mirroring the
//! driver's module path lets the included files' `crate::constants::regs` paths
//! resolve unchanged.

pub mod constants;
pub mod engine;

#[cfg(test)]
mod tests;
