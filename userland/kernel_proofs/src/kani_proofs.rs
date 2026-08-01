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

// The marshalling agreement, over every possible id: the number decode, the
// registry id lookup, and the registry name lookup accept exactly the same
// set of untrusted u64 values, and an accepted id resolves to an entry whose
// id round-trips. A syscall id can never decode to a number the dispatch
// table does not route, and never routes without a decodable number.
#[kani::proof]
fn syscall_id_decode_and_registry_agree_for_all_ids() {
    let id: u64 = kani::any();
    let decoded = crate::syscall::numbers::SyscallNumber::from_u64(id);
    let by_id = crate::syscall::abi::lookup_id(id);
    let by_name = crate::syscall::abi::lookup_name(id);
    assert!(decoded.is_some() == by_id.is_some());
    assert!(decoded.is_some() == by_name.is_some());
    if let (Some(d), Some(r)) = (decoded, by_id) {
        assert!(d == r);
        assert!(d as u64 == id);
    }
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

// The functional differentials: the real kernel functions equal the
// executable spec for every input, not only the sampled ones. These are the
// harnesses that make the refinement machine-checked end to end: Lean proves
// the spec's properties, Verus proves the bit logic computes the spec, and
// these prove the shipping functions are that logic.

// For every 64-bit token and every capability the kernel defines: the real
// has/add/remove equal the spec through the real bit assignment.
#[kani::proof]
fn capability_bits_equal_the_spec_for_all_tokens() {
    let token: u64 = kani::any();
    for cap in crate::capabilities::Capability::all() {
        let bit = crate::capabilities::bit_of(cap);
        assert!(crate::capabilities::bits::has_capability(token, cap) == crate::spec::has(token, bit));
        assert!(crate::capabilities::bits::add_capability(token, cap) == crate::spec::grant(token, bit));
        assert!(
            crate::capabilities::bits::remove_capability(token, cap)
                == crate::spec::revoke(token, bit)
        );
    }
}

// For every address and length: the real check_range equals the spec, exact
// error variants included.
#[kani::proof]
fn check_range_equals_the_spec_for_all_inputs() {
    let addr: u64 = kani::any();
    let len: usize = kani::any();
    assert!(crate::usercopy::check(addr, len) == crate::spec::check_range(addr, len));
}

// For every permission bit pattern: the real PTE encoding and the W^X
// predicate equal the spec.
#[kani::proof]
fn pte_encoding_equals_the_spec_for_all_permissions() {
    let perm: u32 = kani::any();
    let p = PagePermissions::from_bits(perm);
    assert!(p.to_pte_flags() == crate::spec::pte_flags(perm));
    assert!(p.is_wx_violation() == crate::spec::wx_violation(perm));
}

// A PCI BAR decodes an address whose low bits are hardwired to zero, so a base
// that is not a multiple of the window size makes the device answer on an
// address nobody assigned it. For every cursor, limit and size the allocator
// can be handed: whatever it returns is aligned to the size, starts at or
// after the cursor, and ends inside the window.
#[kani::proof]
fn pci_window_carve_is_aligned_and_in_bounds() {
    let cursor: u64 = kani::any();
    let limit: u64 = kani::any();
    let size: u64 = kani::any();

    if let Some((base, next)) = crate::bus::carve(cursor, limit, size) {
        assert!(size.is_power_of_two());
        assert_eq!(base & (size - 1), 0);
        assert!(base >= cursor);
        assert_eq!(next, base + size);
        assert!(next <= limit);
    }
}

// The allocator never hands out a window twice: feeding its own answer back as
// the next cursor always lands past the end of the block it just returned.
#[kani::proof]
fn pci_window_carve_does_not_overlap() {
    let cursor: u64 = kani::any();
    let limit: u64 = kani::any();
    let first: u64 = kani::any();
    let second: u64 = kani::any();

    if let Some((base_a, next_a)) = crate::bus::carve(cursor, limit, first) {
        if let Some((base_b, _)) = crate::bus::carve(next_a, limit, second) {
            assert!(base_b >= base_a + first);
        }
    }
}
