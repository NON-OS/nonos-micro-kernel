// NONOS Operating System (AGPL-3.0-or-later)
//! Both descriptor backends, included from the kernel tree.

pub mod aarch64;

#[path = "../../../../src/arch/paging/descriptor/flags.rs"]
pub mod flags;
#[path = "../../../../src/arch/paging/descriptor/x86_64.rs"]
pub mod x86_64;
