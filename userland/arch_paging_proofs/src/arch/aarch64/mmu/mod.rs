// NONOS Operating System (AGPL-3.0-or-later)
//! The module path the aarch64 encoder reaches for its memory-type table,
//! reconstructed so the encoder is included unmodified and stays bound to the
//! same table MAIR_EL1 is built from.

#[path = "../../../../../../src/arch/aarch64/mmu/attributes/kind.rs"]
mod kind;

pub use kind::MemoryType;
