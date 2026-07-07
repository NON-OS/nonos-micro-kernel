// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::memory::paging::constants::{
    PERM_EXECUTE, PERM_USER, PERM_WRITE, PTE_NO_EXECUTE, PTE_PRESENT, PTE_USER, PTE_WRITABLE,
};
use crate::memory::paging::types::permissions::PagePermissions;

fn perm(bits: u32) -> PagePermissions {
    PagePermissions::from_bits(bits)
}

fn pte_writable(flags: u64) -> bool {
    flags & PTE_WRITABLE != 0
}

fn pte_executable(flags: u64) -> bool {
    flags & PTE_NO_EXECUTE == 0
}

#[test]
fn is_wx_violation_is_exactly_write_and_execute() {
    assert!(!perm(0).is_wx_violation());
    assert!(!perm(PERM_WRITE).is_wx_violation());
    assert!(!perm(PERM_EXECUTE).is_wx_violation());
    assert!(perm(PERM_WRITE | PERM_EXECUTE).is_wx_violation());
    for bits in 0u32..256 {
        let expect = (bits & PERM_WRITE != 0) && (bits & PERM_EXECUTE != 0);
        assert_eq!(perm(bits).is_wx_violation(), expect);
    }
}

#[test]
fn to_pte_flags_encodes_permissions_faithfully() {
    for bits in 0u32..256 {
        let flags = perm(bits).to_pte_flags();
        assert!(flags & PTE_PRESENT != 0, "present is always set");
        assert_eq!(flags & PTE_WRITABLE != 0, bits & PERM_WRITE != 0, "writable bit");
        assert_eq!(flags & PTE_USER != 0, bits & PERM_USER != 0, "user bit");
        assert_eq!(
            flags & PTE_NO_EXECUTE == 0,
            bits & PERM_EXECUTE != 0,
            "executable iff the EXECUTE permission is set"
        );
    }
}

#[test]
fn enforcing_no_wx_violation_yields_a_non_executable_writable_page() {
    // The isolation lemma: a permission set that is not a W^X violation never
    // encodes a page-table entry that is simultaneously writable and
    // executable. So a mapper that rejects `is_wx_violation()` cannot install a
    // W+X page.
    for bits in 0u32..1024 {
        let p = perm(bits);
        if !p.is_wx_violation() {
            let flags = p.to_pte_flags();
            assert!(
                !(pte_writable(flags) && pte_executable(flags)),
                "a non-W^X permission produced a writable+executable PTE"
            );
        }
    }
    // Conversely, a W^X permission does encode a W+X page, which is exactly why
    // the mapper must reject it.
    let wx = perm(PERM_WRITE | PERM_EXECUTE).to_pte_flags();
    assert!(pte_writable(wx) && pte_executable(wx));
}
