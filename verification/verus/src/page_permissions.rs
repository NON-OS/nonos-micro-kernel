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

use vstd::prelude::*;

verus! {

pub open spec fn present_bit() -> u64 { 1u64 << 0 }
pub open spec fn writable_bit() -> u64 { 1u64 << 1 }
pub open spec fn user_bit() -> u64 { 1u64 << 2 }
pub open spec fn cache_disable_bit() -> u64 { 1u64 << 4 }
pub open spec fn no_execute_bit() -> u64 { 1u64 << 63 }
pub open spec fn addr_mask() -> u64 { 0x000F_FFFF_FFFF_F000u64 }

pub open spec fn to_pte(
    phys: u64,
    writable: bool,
    user: bool,
    executable: bool,
    cache_disabled: bool,
) -> u64 {
    (phys & addr_mask())
    | present_bit()
    | if writable { writable_bit() } else { 0 }
    | if user { user_bit() } else { 0 }
    | if cache_disabled { cache_disable_bit() } else { 0 }
    | if executable { 0 } else { no_execute_bit() }
}

pub open spec fn has(raw: u64, bit: u64) -> bool {
    (raw & bit) != 0
}

pub open spec fn executable(raw: u64) -> bool {
    !(has(raw, no_execute_bit()))
}

pub open spec fn wx_violation(raw: u64) -> bool {
    has(raw, writable_bit()) && executable(raw)
}

pub open spec fn permission_subset(child: u64, parent: u64) -> bool {
    (!(has(child, writable_bit())) || has(parent, writable_bit()))
    && (!(has(child, user_bit())) || has(parent, user_bit()))
    && (!executable(child) || executable(parent))
}

pub proof fn encoded_pte_is_present(phys: u64, w: bool, u: bool, x: bool, cd: bool)
    ensures
        has(to_pte(phys, w, u, x, cd), present_bit()),
{
    assert(has(to_pte(phys, w, u, x, cd), present_bit())) by (bit_vector);
}

pub proof fn writable_bit_matches_permission(phys: u64, w: bool, u: bool, x: bool, cd: bool)
    ensures
        has(to_pte(phys, w, u, x, cd), writable_bit()) == w,
{
    assert(has(to_pte(phys, w, u, x, cd), writable_bit()) == w) by (bit_vector);
}

pub proof fn user_bit_matches_permission(phys: u64, w: bool, u: bool, x: bool, cd: bool)
    ensures
        has(to_pte(phys, w, u, x, cd), user_bit()) == u,
{
    assert(has(to_pte(phys, w, u, x, cd), user_bit()) == u) by (bit_vector);
}

pub proof fn executable_bit_matches_permission(phys: u64, w: bool, u: bool, x: bool, cd: bool)
    ensures
        executable(to_pte(phys, w, u, x, cd)) == x,
{
    assert(executable(to_pte(phys, w, u, x, cd)) == x) by (bit_vector);
}

pub proof fn no_wx_when_permission_rejects_wx(phys: u64, w: bool, u: bool, x: bool, cd: bool)
    requires
        !(w && x),
    ensures
        !wx_violation(to_pte(phys, w, u, x, cd)),
{
    writable_bit_matches_permission(phys, w, u, x, cd);
    executable_bit_matches_permission(phys, w, u, x, cd);
}

pub proof fn permission_subset_is_monotonic(child: u64, parent: u64)
    requires
        permission_subset(child, parent),
    ensures
        has(child, writable_bit()) ==> has(parent, writable_bit()),
        has(child, user_bit()) ==> has(parent, user_bit()),
        executable(child) ==> executable(parent),
{
}

} // verus!
