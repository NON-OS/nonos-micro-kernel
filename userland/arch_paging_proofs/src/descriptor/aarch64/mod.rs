// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../../../src/arch/paging/descriptor/aarch64/bits.rs"]
pub mod bits;
#[path = "../../../../../src/arch/paging/descriptor/aarch64/build.rs"]
pub mod build;
#[path = "../../../../../src/arch/paging/descriptor/aarch64/read.rs"]
pub mod read;

pub use bits::ADDR_MASK;
pub use build::{leaf, table};
pub use read::{address, is_block, is_present, is_user, is_writable, table_grants_user};
