// NONOS Operating System (AGPL-3.0-or-later)
// Only the permission and PTE-flag constants the permission logic needs. The
// rest of the kernel constants module carries page-walk helpers this proof
// does not exercise, so they are not included (nothing dead is pulled in).
#[path = "../../../../../../src/memory/paging/constants/permissions.rs"]
mod permissions;
#[path = "../../../../../../src/memory/paging/constants/pte_flags.rs"]
mod pte_flags;
pub use permissions::*;
pub use pte_flags::*;
