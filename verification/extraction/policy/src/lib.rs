// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root for the user-copy range policy and the page-permission
//! encoding. The real kernel source is included via #[path]; the two
//! functions below are call-graph roots that only forward to the real
//! methods so the extractor has plain function names to start from.

pub mod arch;
pub mod memory;
pub mod usercopy;

use memory::paging::types::permissions::PagePermissions;

pub fn to_pte_flags(p: PagePermissions) -> u64 {
    p.to_pte_flags()
}

pub fn is_wx_violation(p: PagePermissions) -> bool {
    p.is_wx_violation()
}
