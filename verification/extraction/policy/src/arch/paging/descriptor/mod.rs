// NONOS Operating System (AGPL-3.0-or-later)
// The descriptor boundary the paging constants resolve against. Both halves
// come from the kernel tree, so the constants and predicates under extraction
// are the ones the kernel compiles.
#[path = "../../../../../../../src/arch/paging/descriptor/flags.rs"]
pub mod flags;

#[path = "../../../../../../../src/arch/paging/descriptor/x86_64.rs"]
mod backend;

pub use backend::*;
