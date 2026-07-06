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

//! Kani harnesses: the isolation invariants hold for every permission bit
//! pattern and every address/length, not just the sampled ones.

use crate::memory::paging::constants::{PERM_EXECUTE, PERM_WRITE, PTE_NO_EXECUTE, PTE_WRITABLE};
use crate::memory::paging::types::permissions::PagePermissions;
use crate::usercopy::{accepts, MAX_COPY_SIZE, USER_SPACE_END};

// For every permission set: `is_wx_violation` is exactly write&&execute, and a
// non-violating set never encodes a writable+executable page-table entry.
#[kani::proof]
fn wx_isolation_holds_for_all_permissions() {
    let bits: u32 = kani::any();
    let p = PagePermissions::from_bits(bits);

    assert_eq!(
        p.is_wx_violation(),
        (bits & PERM_WRITE != 0) && (bits & PERM_EXECUTE != 0)
    );

    let flags = p.to_pte_flags();
    let writable = flags & PTE_WRITABLE != 0;
    let executable = flags & PTE_NO_EXECUTE == 0;
    if !p.is_wx_violation() {
        assert!(!(writable && executable));
    }
}

// Decoding an untrusted syscall id is total: no u64 value panics or accesses
// out of bounds while scanning the ABI registry.
#[kani::proof]
fn syscall_decode_is_total() {
    let id: u64 = kani::any();
    let _ = crate::syscall::numbers::SyscallNumber::from_u64(id);
}

// For every address/length: `check_range` is total, and an accepted non-empty
// range lies inside user space without wrapping.
#[kani::proof]
fn user_range_check_is_total_and_bounded() {
    let addr: u64 = kani::any();
    let len: usize = kani::any();

    if accepts(addr, len) {
        assert!(addr != 0);
        assert!(len <= MAX_COPY_SIZE);
        if len > 0 {
            let end = addr.checked_add(len as u64 - 1);
            assert!(end.is_some());
            assert!(end.unwrap() <= USER_SPACE_END);
        }
    }
}
