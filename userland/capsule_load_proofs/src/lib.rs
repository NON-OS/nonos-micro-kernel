// NONOS Operating System (AGPL-3.0-or-later)
//! Host proof for the runtime capsule-load certificate clock gate. The real
//! gate source is included via #[path] so the test pins production behavior.

#[path = "../../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/validity_clock.rs"]
pub mod validity_clock;

#[cfg(test)]
mod tests;
