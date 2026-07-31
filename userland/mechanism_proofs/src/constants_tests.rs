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

//! Holds the kernel's constants against the numbers the Lean specs state.
//! A spec writes them as its own literals, so the two agree only until someone
//! edits one. Each assertion names the definition it mirrors; changing the
//! kernel constant fails here until the Lean file moves with it.

use crate::constants::canonical::{CANONICAL_HIGH_MIN, CANONICAL_LOW_MAX, USER_TOP};
use crate::constants::chain::MAX_CHAIN_DEPTH;
use crate::constants::ipc_limits::MAX_MESSAGE_SIZE;
use crate::constants::page_sizes::{PAGE_SIZE_1G, PAGE_SIZE_2M, PAGE_SIZE_4K};

#[test]
fn the_user_half_matches_the_specifications() {
    // Nonos/UserCopy.lean userEnd, Nonos/DemandPaging.lean userTop,
    // Nonos/Isolation.lean userEnd.
    assert_eq!(CANONICAL_LOW_MAX, 0x0000_7FFF_FFFF_FFFF);
    assert_eq!(USER_TOP, 0x0000_7FFF_FFFF_FFFF);
    // Nonos/UserWalk.lean reasons about the split rather than the number, but
    // the kernel half has to start where the user half stops.
    assert_eq!(CANONICAL_HIGH_MIN, 0xFFFF_8000_0000_0000);
    assert_eq!(USER_TOP.wrapping_add(1).leading_zeros(), 16);
}

#[test]
fn the_page_sizes_match_the_specifications() {
    // Nonos/DemandPaging.lean pageSize. The walk in Nonos/UserWalk.lean stops
    // at three levels whose sizes are these.
    assert_eq!(PAGE_SIZE_4K, 4096);
    assert_eq!(PAGE_SIZE_2M, 2 * 1024 * 1024);
    assert_eq!(PAGE_SIZE_1G, 1024 * 1024 * 1024);
    // Each level covers 512 of the one below it, which is what makes the nine
    // bits per index in the walker right.
    assert_eq!(PAGE_SIZE_2M, PAGE_SIZE_4K * 512);
    assert_eq!(PAGE_SIZE_1G, PAGE_SIZE_2M * 512);
}

#[test]
fn the_ipc_message_bound_matches_the_specification() {
    // Nonos/Ipc.lean maxMessageSize.
    assert_eq!(MAX_MESSAGE_SIZE, 1024 * 1024);
}

#[test]
fn the_delegation_chain_depth_is_bounded() {
    // No Lean file states this one. It is pinned here so that raising it is a
    // deliberate edit rather than a silent one, and so the gap is visible
    // instead of implied: verify_chain enforces it, and nothing proves that a
    // chain at the bound still attenuates. Zero would refuse every delegation,
    // which the value below rules out.
    assert_eq!(MAX_CHAIN_DEPTH, 16);
}
