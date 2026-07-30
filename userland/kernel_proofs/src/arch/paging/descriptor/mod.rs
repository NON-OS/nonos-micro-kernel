// NONOS Operating System (AGPL-3.0-or-later)
// The descriptor boundary the kernel's PTE flag constants derive from. Both
// halves come from the kernel tree so the constants under proof are the ones
// the kernel compiles, not a restatement of them.
#[path = "../../../../../../src/arch/paging/descriptor/flags.rs"]
pub mod flags;

#[path = "../../../../../../src/arch/paging/descriptor/x86_64.rs"]
mod backend;

pub use backend::*;
